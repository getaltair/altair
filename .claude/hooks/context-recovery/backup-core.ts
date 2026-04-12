import {
	existsSync,
	mkdirSync,
	readdirSync,
	readFileSync,
	writeFileSync,
} from "node:fs";
import { homedir } from "node:os";
import { join } from "node:path";

const projectDir = process.cwd();

export interface TranscriptSummary {
	userRequests: string[];
	filesModified: string[];
	tasksCreated: { subject: string; description: string }[];
	tasksCompleted: { taskId: string }[];
	subAgentCalls: { agent: string; description: string }[];
	skillsLoaded: string[];
	buildTestResults: { command: string; result: string }[];
	sessionStart: string | null;
	sessionEnd: string | null;
}

interface BackupState {
	sessionId: string | null;
	currentBackupPath: string | null;
	lastUpdated: string | null;
	lastBackupThreshold?: number | null;
}

function getStatePath(): string {
	return join(homedir(), ".claude", "cortex-backup-state.json");
}

export function readState(): BackupState {
	try {
		const p = getStatePath();
		if (existsSync(p)) return JSON.parse(readFileSync(p, "utf-8"));
	} catch {
		/* ignore */
	}
	return { sessionId: null, currentBackupPath: null, lastUpdated: null };
}

export function writeState(state: BackupState): void {
	try {
		const dir = join(homedir(), ".claude");
		mkdirSync(dir, { recursive: true });
		writeFileSync(getStatePath(), JSON.stringify(state, null, 2));
	} catch {
		/* fail silently */
	}
}

export function parseTranscript(
	transcriptPath: string,
): TranscriptSummary | null {
	if (!existsSync(transcriptPath)) return null;

	const summary: TranscriptSummary = {
		userRequests: [],
		filesModified: [],
		tasksCreated: [],
		tasksCompleted: [],
		subAgentCalls: [],
		skillsLoaded: [],
		buildTestResults: [],
		sessionStart: null,
		sessionEnd: null,
	};

	try {
		const lines = readFileSync(transcriptPath, "utf-8")
			.split("\n")
			.filter((l) => l.trim());
		const seenFiles = new Set<string>();
		const seenSkills = new Set<string>();

		for (const line of lines) {
			try {
				const entry = JSON.parse(line);
				if (entry.timestamp) {
					if (!summary.sessionStart) summary.sessionStart = entry.timestamp;
					summary.sessionEnd = entry.timestamp;
				}

				if (
					entry.type === "user" &&
					typeof entry.message?.content === "string"
				) {
					const t = entry.message.content.trim();
					if (
						t &&
						t.length >= 10 &&
						!t.startsWith("[{") &&
						!t.startsWith('{"tool_use_id"')
					) {
						summary.userRequests.push(t);
					}
				}

				if (
					entry.type === "assistant" &&
					Array.isArray(entry.message?.content)
				) {
					for (const block of entry.message.content) {
						if (block.type === "tool_use") {
							const { name, input } = block;
							if ((name === "Write" || name === "Edit") && input?.file_path) {
								seenFiles.add(input.file_path);
							}
							if (name === "TaskCreate")
								summary.tasksCreated.push({
									subject: input?.subject ?? "",
									description: input?.description ?? "",
								});
							if (name === "TaskUpdate" && input?.status === "completed")
								summary.tasksCompleted.push({ taskId: input.taskId });
							if (name === "Task")
								summary.subAgentCalls.push({
									agent: input?.subagent_type ?? "unknown",
									description: input?.description ?? "",
								});
							if (name === "Skill" && input?.skill) seenSkills.add(input.skill);
							if (name === "Bash" && input?.command) {
								const cmd = input.command.toLowerCase();
								if (cmd.includes("build") || cmd.includes("test"))
									summary.buildTestResults.push({
										command: input.command,
										result: "executed",
									});
							}
						}
					}
				}
			} catch {}
		}

		summary.filesModified = Array.from(seenFiles);
		summary.skillsLoaded = Array.from(seenSkills);
		return summary;
	} catch {
		return null;
	}
}

export function formatMarkdown(
	summary: TranscriptSummary,
	trigger: string,
	sessionId: string,
	contextPct?: number,
): string {
	const lines: string[] = [
		`# Session Backup`,
		"",
		`**Session ID:** ${sessionId}`,
		`**Trigger:** ${trigger}`,
	];
	if (contextPct !== undefined)
		lines.push(`**Context Remaining:** ${contextPct}%`);
	lines.push(`**Generated:** ${new Date().toISOString()}`);
	if (summary.sessionStart)
		lines.push(`**Session Start:** ${summary.sessionStart}`);
	lines.push("");

	if (summary.userRequests.length) {
		lines.push("## User Requests");
		summary.userRequests.forEach((r) => {
			lines.push(`- ${r}`);
		});
		lines.push("");
	}
	if (summary.filesModified.length) {
		lines.push("## Files Modified");
		summary.filesModified.forEach((f) => {
			lines.push(`- \`${f}\``);
		});
		lines.push("");
	}
	if (summary.tasksCreated.length) {
		lines.push("## Tasks Created");
		summary.tasksCreated.forEach((t) => {
			lines.push(`- **${t.subject}**: ${t.description}`);
		});
		lines.push("");
	}
	if (summary.tasksCompleted.length) {
		lines.push(`## Tasks Completed: ${summary.tasksCompleted.length}`);
		lines.push("");
	}
	if (summary.subAgentCalls.length) {
		lines.push("## Sub-Agents Invoked");
		summary.subAgentCalls.forEach((c) => {
			lines.push(`- **${c.agent}**: ${c.description}`);
		});
		lines.push("");
	}
	if (summary.skillsLoaded.length) {
		lines.push("## Skills Loaded");
		summary.skillsLoaded.forEach((s) => {
			lines.push(`- ${s}`);
		});
		lines.push("");
	}

	// Include .cortex/session.md content if it exists
	const sessionFile = join(projectDir, ".cortex", "session.md");
	if (existsSync(sessionFile)) {
		lines.push("## Execution Session State");
		lines.push(readFileSync(sessionFile, "utf-8"));
		lines.push("");
	}

	// Include tail of .cortex/events.log (wave coordination channel)
	const eventsFile = join(projectDir, ".cortex", "events.log");
	if (existsSync(eventsFile)) {
		try {
			const raw = readFileSync(eventsFile, "utf-8");
			const eventLines = raw.split("\n").filter((l) => l.trim());
			const tail = eventLines.slice(-50);
			if (tail.length > 0) {
				lines.push(
					`## Events Log (last ${tail.length} of ${eventLines.length})`,
				);
				lines.push("```");
				for (const l of tail) lines.push(l);
				lines.push("```");
				lines.push("");
			}
		} catch {
			/* ignore */
		}
	}

	// Include list of rendered specialist prompts so recovery can find them
	const promptsDir = join(projectDir, ".cortex", "prompts");
	if (existsSync(promptsDir)) {
		try {
			const files = readdirSync(promptsDir)
				.filter((f) => f.endsWith(".md"))
				.sort();
			if (files.length > 0) {
				lines.push(`## Rendered Specialist Prompts: ${files.length}`);
				for (const f of files) lines.push(`- \`.cortex/prompts/${f}\``);
				lines.push("");
			}
		} catch {
			/* ignore */
		}
	}

	// Include Steps.md progress if active feature exists
	const contextDir = join(projectDir, "Context", "Features");
	if (existsSync(contextDir)) {
		try {
			const features = readdirSync(contextDir)
				.filter((f) => f.match(/^\d{3}/))
				.sort()
				.reverse();
			for (const feat of features) {
				const stepsPath = join(contextDir, feat, "Steps.md");
				if (existsSync(stepsPath)) {
					const content = readFileSync(stepsPath, "utf-8");
					if (content.includes("In progress")) {
						lines.push(`## Active Steps.md: ${feat}`);
						const progressMatch = content.match(/## Progress[\s\S]*?(?=\n## )/);
						if (progressMatch) lines.push(progressMatch[0]);
						lines.push("");
						break;
					}
				}
			}
		} catch {
			/* ignore */
		}
	}

	return lines.join("\n");
}

function getOrdinal(day: number): string {
	if (day > 3 && day < 21) return "th";
	switch (day % 10) {
		case 1:
			return "st";
		case 2:
			return "nd";
		case 3:
			return "rd";
		default:
			return "th";
	}
}

function friendlyDate(d: Date): string {
	const months = [
		"Jan",
		"Feb",
		"Mar",
		"Apr",
		"May",
		"Jun",
		"Jul",
		"Aug",
		"Sep",
		"Oct",
		"Nov",
		"Dec",
	];
	const day = d.getDate();
	let h = d.getHours();
	const m = d.getMinutes().toString().padStart(2, "0");
	const ampm = h >= 12 ? "pm" : "am";
	h = h % 12 || 12;
	return `${months[d.getMonth()]} ${day}${getOrdinal(day)} ${h}:${m}${ampm}`;
}

export function saveBackup(
	markdown: string,
	existingPath?: string | null,
): { fullPath: string; relativePath: string } | null {
	try {
		const backupDir = join(projectDir, ".cortex", "backups");
		mkdirSync(backupDir, { recursive: true });

		if (existingPath) {
			const full = join(projectDir, existingPath);
			if (existsSync(full)) {
				writeFileSync(full, markdown);
				return { fullPath: full, relativePath: existingPath };
			}
		}

		const existing = existsSync(backupDir)
			? readdirSync(backupDir).filter(
					(f) => f.endsWith(".md") && /^\d+-/.test(f),
				)
			: [];
		const nums = existing.map((f) =>
			parseInt(f.match(/^(\d+)-/)?.[1] ?? "0", 10),
		);
		const next = nums.length ? Math.max(...nums) + 1 : 1;
		const name = `${next}-backup-${friendlyDate(new Date())}.md`;
		const fullPath = join(backupDir, name);
		const relativePath = `.cortex/backups/${name}`;

		writeFileSync(fullPath, markdown);
		return { fullPath, relativePath };
	} catch {
		return null;
	}
}

export function findTranscript(sessionId: string): string | null {
	try {
		const claudeDir = join(homedir(), ".claude", "projects");
		if (!existsSync(claudeDir)) return null;
		for (const dir of readdirSync(claudeDir)) {
			const p = join(claudeDir, dir, `${sessionId}.jsonl`);
			if (existsSync(p)) return p;
		}
		return null;
	} catch {
		return null;
	}
}

// Regex markers for extracting project conventions from assistant text.
// Matches lines like "Decision: use X", "Convention: always Y", ADR refs,
// and tech-choice phrases ("chose X over Y", "using X instead of Y").
const PATTERN_MARKER =
	/\b(?:Decision|Convention|Pattern|Prefer|Always|Never)\s*:\s*.+/i;
const ADR_REF = /\bADR-\d+\b/;
const TECH_CHOICE =
	/\b(?:chose|using|switched to|prefer(?:ring)?)\s+[\w-.]+\s+(?:over|instead of)\s+[\w-.]+/i;

const PATTERNS_FILE = ".cortex/patterns.md";
const PATTERNS_MAX_LINES = 200;
const PATTERNS_HEADER = "# Session Patterns\n\n";

/**
 * Extract project-convention lines from an assistant transcript.
 * Regex-only -- no LLM calls, no token cost.
 */
export function extractPatterns(transcriptPath: string): string[] {
	if (!existsSync(transcriptPath)) return [];

	const found: string[] = [];
	const seen = new Set<string>();

	try {
		const rawLines = readFileSync(transcriptPath, "utf-8")
			.split("\n")
			.filter((l) => l.trim());

		for (const rawLine of rawLines) {
			try {
				const entry = JSON.parse(rawLine);
				if (
					entry.type !== "assistant" ||
					!Array.isArray(entry.message?.content)
				)
					continue;

				for (const block of entry.message.content) {
					if (block.type !== "text" || !block.text) continue;

					for (const textLine of (block.text as string).split("\n")) {
						const trimmed = textLine.trim();
						if (!trimmed || trimmed.length > 300) continue;

						const isPattern =
							PATTERN_MARKER.test(trimmed) ||
							ADR_REF.test(trimmed) ||
							TECH_CHOICE.test(trimmed);

						if (isPattern && !seen.has(trimmed)) {
							seen.add(trimmed);
							found.push(trimmed.slice(0, 200));
						}
					}
				}
			} catch {
				/* skip malformed lines */
			}
		}
	} catch {
		/* fail silently */
	}

	return found;
}

/**
 * Append extracted patterns to .cortex/patterns.md.
 * Caps the file at PATTERNS_MAX_LINES to prevent unbounded growth.
 */
export function appendPatterns(patterns: string[], sessionId: string): void {
	if (patterns.length === 0) return;

	try {
		const patternsFile = join(projectDir, PATTERNS_FILE);
		const timestamp = new Date().toISOString();
		const shortId = sessionId.slice(0, 8);

		const newEntry =
			`## ${timestamp} (session ${shortId})\n` +
			patterns.map((p) => `- ${p}`).join("\n") +
			"\n\n";

		let existing = existsSync(patternsFile)
			? readFileSync(patternsFile, "utf-8")
			: PATTERNS_HEADER;

		if (!existing.startsWith("# Session Patterns")) {
			existing = PATTERNS_HEADER + existing;
		}

		const combined = existing + newEntry;
		const allLines = combined.split("\n");

		let content: string;
		if (allLines.length > PATTERNS_MAX_LINES) {
			// Preserve header, roll off oldest entries from the body
			const body = allLines.slice(PATTERNS_HEADER.split("\n").length - 1);
			const capped = body.slice(body.length - (PATTERNS_MAX_LINES - 3));
			content = PATTERNS_HEADER + capped.join("\n");
		} else {
			content = combined;
		}

		mkdirSync(join(projectDir, ".cortex"), { recursive: true });
		writeFileSync(patternsFile, content);
	} catch {
		/* fail silently -- never block backup */
	}
}

export function runBackup(
	sessionId: string,
	trigger: string,
	transcriptPath?: string,
	contextPct?: number,
): string | null {
	const actual = transcriptPath ?? findTranscript(sessionId);
	if (!actual) return null;

	const summary = parseTranscript(actual);
	if (!summary) return null;

	const markdown = formatMarkdown(summary, trigger, sessionId, contextPct);
	const state = readState();
	const existing =
		state.sessionId === sessionId ? state.currentBackupPath : null;
	const result = saveBackup(markdown, existing);
	if (!result) return null;

	writeState({
		sessionId,
		currentBackupPath: result.relativePath,
		lastUpdated: new Date().toISOString(),
	});

	// Extract and persist project conventions from the transcript (regex-only, no cost).
	const patterns = extractPatterns(actual);
	appendPatterns(patterns, sessionId);

	return result.relativePath;
}
