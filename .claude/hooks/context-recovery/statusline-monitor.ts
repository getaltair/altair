/**
 * StatusLine hook: renders the Cortex status bar and triggers
 * context-recovery backups at 30/15/5% remaining thresholds.
 *
 * Schema reference: code.claude.com/docs/en/statusline
 * The documented context field is context_window.remaining_percentage;
 * free_until_compact is kept as a legacy fallback.
 */
import { execFileSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { homedir } from "node:os";
import { basename, join } from "node:path";
import { readState, runBackup, writeState } from "./backup-core.ts";

interface StatusInput {
	session_id: string;
	transcript_path: string;
	cwd?: string;
	model?: { id?: string; display_name?: string };
	workspace?: {
		current_dir?: string;
		project_dir?: string;
		git_worktree?: string;
	};
	cost?: {
		total_cost_usd?: number;
		total_lines_added?: number;
		total_lines_removed?: number;
	};
	context_window?: {
		used_percentage?: number | null;
		remaining_percentage?: number | null;
	};
	vim?: { mode?: string };
	output_style?: { name?: string };
	free_until_compact?: number;
}

const THRESHOLDS = [30, 15, 5];

// ---------- ANSI helpers ----------
const ESC = "\x1b[";
const RESET = `${ESC}0m`;
const wrap = (code: string, s: string) => `${ESC}${code}m${s}${RESET}`;
const dim = (s: string) => wrap("2", s);
const bold = (s: string) => wrap("1", s);
const cyan = (s: string) => wrap("36", s);
const green = (s: string) => wrap("32", s);
const yellow = (s: string) => wrap("33", s);
const red = (s: string) => wrap("31", s);
const magenta = (s: string) => wrap("35", s);
const blue = (s: string) => wrap("34", s);

// ---------- Git helpers ----------
// execFileSync (no shell) with a fixed argv list prevents injection even
// though all args here are hardcoded. Timeout guards a hung git process.
function git(cwd: string, args: string[]): string | null {
	try {
		return execFileSync("git", args, {
			cwd,
			stdio: ["ignore", "pipe", "ignore"],
			encoding: "utf-8",
			timeout: 200,
		}).trim();
	} catch {
		return null;
	}
}

// ---------- Effort level ----------
// Effort is not in the statusline stdin payload, so we reconstruct it from:
// 1. ~/.claude/history.jsonl — scan backward for the last `/effort <level>`
//    invocation in the current session (captures `/effort max` overrides).
// 2. ~/.claude/settings.json — `effortLevel` as the persistent default.
// Returns null if neither source yields a value. Any I/O error is swallowed.
function readSessionEffort(sessionId: string): string | null {
	try {
		const path = join(homedir(), ".claude", "history.jsonl");
		if (!existsSync(path)) return null;
		const text = readFileSync(path, "utf-8");
		const lines = text.split("\n");
		for (let i = lines.length - 1; i >= 0; i--) {
			const line = lines[i];
			if (!line || !line.includes(sessionId)) continue;
			if (!line.includes("/effort")) continue;
			try {
				const entry = JSON.parse(line) as {
					sessionId?: string;
					display?: string;
				};
				if (entry.sessionId !== sessionId) continue;
				const match = entry.display?.match(/^\/effort\s+(\w+)/);
				if (match) return match[1].toLowerCase();
			} catch {
				/* malformed line, skip */
			}
		}
	} catch {
		/* history unreadable, fall through */
	}
	return null;
}

function readDefaultEffort(): string | null {
	try {
		const path = join(homedir(), ".claude", "settings.json");
		if (!existsSync(path)) return null;
		const parsed = JSON.parse(readFileSync(path, "utf-8")) as {
			effortLevel?: unknown;
		};
		if (typeof parsed.effortLevel === "string") return parsed.effortLevel;
	} catch {
		/* settings unreadable */
	}
	return null;
}

function resolveEffort(sessionId: string): string | null {
	return readSessionEffort(sessionId) ?? readDefaultEffort();
}

// ---------- Segment builders ----------
function segModel(data: StatusInput, effort: string | null): string | null {
	const name = data.model?.display_name;
	if (!name) return null;
	return effort ? dim(`${name} · ${effort}`) : dim(name);
}

function segDir(data: StatusInput): string | null {
	const dir = data.workspace?.current_dir ?? data.cwd;
	if (!dir) return null;
	const home = homedir();
	const display = dir.startsWith(home) ? `~${dir.slice(home.length)}` : dir;
	return cyan(basename(display) || display);
}

function segWorktree(data: StatusInput): string | null {
	const name = data.workspace?.git_worktree;
	if (!name) return null;
	return dim(`wt:${name}`);
}

function segGit(cwd: string | undefined): string | null {
	if (!cwd) return null;
	if (git(cwd, ["rev-parse", "--is-inside-work-tree"]) !== "true") return null;

	let branch = git(cwd, ["symbolic-ref", "--quiet", "--short", "HEAD"]);
	if (!branch) {
		const sha = git(cwd, ["rev-parse", "--short", "HEAD"]);
		branch = sha ? `(${sha})` : "(detached)";
	}

	const parts: string[] = [green(branch)];

	const porcelain = git(cwd, ["status", "--porcelain"]);
	if (porcelain) {
		const count = porcelain.split("\n").filter(Boolean).length;
		if (count > 0) parts.push(yellow(`*${count}`));
	}

	const leftRight = git(cwd, [
		"rev-list",
		"--left-right",
		"--count",
		"HEAD...@{u}",
	]);
	if (leftRight) {
		const [ahead, behind] = leftRight.split(/\s+/).map((n) => Number(n) || 0);
		if (ahead > 0) parts.push(blue(`↑${ahead}`));
		if (behind > 0) parts.push(blue(`↓${behind}`));
	}

	return parts.join(" ");
}

function segDevEnv(cwd: string | undefined): string | null {
	if (process.env.VIRTUAL_ENV) {
		return magenta(`venv(${basename(process.env.VIRTUAL_ENV)})`);
	}
	if (process.env.CONDA_DEFAULT_ENV) {
		return magenta(`conda(${process.env.CONDA_DEFAULT_ENV})`);
	}
	if (!cwd) return null;
	const lockfiles: Array<[string, string]> = [
		["bun.lock", "bun"],
		["bun.lockb", "bun"],
		["pnpm-lock.yaml", "pnpm"],
		["yarn.lock", "yarn"],
		["package-lock.json", "npm"],
		["Cargo.toml", "cargo"],
		["go.mod", "go"],
		["pyproject.toml", "py"],
		["requirements.txt", "py"],
	];
	for (const [file, label] of lockfiles) {
		if (existsSync(join(cwd, file))) return magenta(label);
	}
	return null;
}

function segOutputStyle(data: StatusInput): string | null {
	const name = data.output_style?.name;
	if (!name || name === "default") return null;
	return dim(name);
}

function segContext(remaining: number | undefined): string | null {
	if (remaining === undefined) return null;
	const pct = Math.round(remaining);
	const label = `ctx ${pct}%`;
	if (pct <= 5) return bold(red(label));
	if (pct <= 15) return red(label);
	if (pct <= 30) return yellow(label);
	return green(label);
}

function segCost(data: StatusInput): string | null {
	const usd = data.cost?.total_cost_usd;
	if (!usd || usd <= 0) return null;
	return dim(`$${usd.toFixed(2)}`);
}

function segDiff(data: StatusInput): string | null {
	const added = data.cost?.total_lines_added ?? 0;
	const removed = data.cost?.total_lines_removed ?? 0;
	if (added <= 0 && removed <= 0) return null;
	const parts: string[] = [];
	if (added > 0) parts.push(green(`+${added}`));
	if (removed > 0) parts.push(red(`-${removed}`));
	return parts.join(" ");
}

// ---------- Main ----------
function deriveRemaining(data: StatusInput): number | undefined {
	const cw = data.context_window;
	if (cw?.remaining_percentage != null) return cw.remaining_percentage;
	if (cw?.used_percentage != null) return 100 - cw.used_percentage;
	if (data.free_until_compact != null) return data.free_until_compact;
	return undefined;
}

function maybeBackup(data: StatusInput, remaining: number): void {
	const state = readState();
	const lastThreshold = state.lastBackupThreshold as number | null | undefined;
	let crossed: number | null = null;

	for (const threshold of THRESHOLDS) {
		if (
			remaining <= threshold &&
			(lastThreshold === null ||
				lastThreshold === undefined ||
				lastThreshold > threshold)
		) {
			crossed = threshold;
			break;
		}
	}

	if (crossed !== null) {
		const result = runBackup(
			data.session_id,
			`crossed_${crossed}pct`,
			data.transcript_path,
			remaining,
		);
		if (result) {
			const updated = readState();
			updated.lastBackupThreshold = crossed;
			writeState(updated);
		}
	}
}

function build(data: StatusInput): string {
	const cwd = data.workspace?.current_dir ?? data.cwd;
	const remaining = deriveRemaining(data);
	// Vim mode and the effort indicator already appear in Claude Code's
	// built-in footer, so we omit segVim here and merge effort into segModel.
	const effort = resolveEffort(data.session_id);

	// Segments are grouped left-to-right:
	//   1. Claude identity   — model + effort, output style
	//   2. Location          — dir, worktree, dev env
	//   3. Git               — branch, dirty, ahead/behind
	//   4. Session metrics   — context %, cost, diff
	const segments: Array<string | null> = [
		// Claude identity
		segModel(data, effort),
		segOutputStyle(data),
		// Location
		segDir(data),
		segWorktree(data),
		segDevEnv(cwd),
		// Git
		segGit(cwd),
		// Session metrics
		segContext(remaining),
		segCost(data),
		segDiff(data),
	];

	const sep = ` ${dim("│")} `;
	return segments.filter((s): s is string => s !== null && s !== "").join(sep);
}

try {
	const input = readFileSync(0, "utf-8");
	const data = JSON.parse(input) as StatusInput;

	const remaining = deriveRemaining(data);
	if (remaining !== undefined) {
		try {
			maybeBackup(data, remaining);
		} catch (err) {
			console.error("StatusLine backup error:", err);
		}
	}

	const line = build(data);
	console.log(line || "Cortex");
	process.exit(0);
} catch (err) {
	console.error("StatusLine error:", err);
	console.log("Cortex");
	process.exit(0);
}
