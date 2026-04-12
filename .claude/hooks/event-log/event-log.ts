#!/usr/bin/env bun
import { appendFileSync, existsSync, mkdirSync } from "node:fs";
import { dirname, join } from "node:path";

export type EventType =
	| "task_done"
	| "contract_ready"
	| "milestone_reached"
	| "error"
	| "note";

export interface EventEntry {
	ts: string;
	source: string;
	type: EventType;
	task?: string;
	contract?: string;
	note?: string;
}

const projectDir = process.cwd();
const LOG_PATH = join(projectDir, ".cortex", "events.log");
const VALID_TYPES: readonly EventType[] = [
	"task_done",
	"contract_ready",
	"milestone_reached",
	"error",
	"note",
] as const;

export function appendEvent(
	partial: Omit<EventEntry, "ts"> & { ts?: string },
): EventEntry {
	const entry: EventEntry = {
		ts: partial.ts ?? new Date().toISOString(),
		source: partial.source,
		type: partial.type,
	};
	if (partial.task) entry.task = partial.task;
	if (partial.contract) entry.contract = partial.contract;
	if (partial.note) entry.note = partial.note;

	const dir = dirname(LOG_PATH);
	if (!existsSync(dir)) mkdirSync(dir, { recursive: true });
	appendFileSync(LOG_PATH, `${JSON.stringify(entry)}\n`);
	return entry;
}

export function formatForMonitor(entry: EventEntry): string {
	const parts: string[] = [`[${entry.ts}]`, entry.source, entry.type];
	if (entry.task) parts.push(`task=${entry.task}`);
	if (entry.contract) parts.push(`contract=${entry.contract}`);
	if (entry.note) parts.push(`note="${entry.note}"`);
	return parts.join(" ");
}

function parseArgs(argv: string[]): Record<string, string> {
	const args: Record<string, string> = {};
	for (let i = 0; i < argv.length; i++) {
		const arg = argv[i];
		if (!arg?.startsWith("--")) continue;
		const key = arg.slice(2);
		const next = argv[i + 1];
		if (next && !next.startsWith("--")) {
			args[key] = next;
			i++;
		} else {
			args[key] = "true";
		}
	}
	return args;
}

function isEventType(value: string): value is EventType {
	return (VALID_TYPES as readonly string[]).includes(value);
}

function printUsage(): void {
	console.error(
		"Usage: event-log.ts append --source <name> --type <type> [--task <id>] [--contract <path>] [--note <text>]",
	);
	console.error(`  --type: one of ${VALID_TYPES.join(", ")}`);
}

async function main(): Promise<void> {
	const [subcommand, ...rest] = process.argv.slice(2);

	if (subcommand !== "append") {
		printUsage();
		process.exit(1);
	}

	const args = parseArgs(rest);

	if (!args.source || !args.type) {
		console.error("Required: --source and --type");
		printUsage();
		process.exit(1);
	}

	if (!isEventType(args.type)) {
		console.error(`Invalid --type. Must be one of: ${VALID_TYPES.join(", ")}`);
		process.exit(1);
	}

	const entry = appendEvent({
		source: args.source,
		type: args.type,
		task: args.task,
		contract: args.contract,
		note: args.note,
	});

	console.log(formatForMonitor(entry));
}

if (import.meta.main) {
	main().catch((err: unknown) => {
		console.error(err);
		process.exit(1);
	});
}
