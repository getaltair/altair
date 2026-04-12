#!/usr/bin/env bun
import { readFileSync } from "node:fs";

const API_URL = "https://api.anthropic.com/v1/messages";
const BETA_HEADER = "advisor-tool-2026-03-01";
const ANTHROPIC_VERSION = "2023-06-01";
const EXECUTOR_MODEL = "claude-sonnet-4-6";
const ADVISOR_MODEL = "claude-opus-4-6";
const MAX_TOKENS = 1024;

const SYSTEM_PROMPT = `The advisor should respond in under 100 words and use enumerated steps, not explanations.

You have access to an \`advisor\` tool backed by a stronger reviewer model. Call the advisor tool before responding so your final answer reflects its guidance.

When you respond, provide ONLY the enumerated steps -- no preamble, no commentary on having consulted the advisor. Every response must be <=100 words total.`;

interface TextBlock {
	type: "text";
	text: string;
}

interface ServerToolUseBlock {
	type: "server_tool_use";
	id: string;
	name: string;
	input: Record<string, unknown>;
}

interface AdvisorResultPayload {
	type: "advisor_result";
	text: string;
}

interface AdvisorRedactedResultPayload {
	type: "advisor_redacted_result";
	encrypted_content: string;
}

interface AdvisorErrorPayload {
	type: "advisor_tool_result_error";
	error_code: string;
}

interface AdvisorToolResultBlock {
	type: "advisor_tool_result";
	tool_use_id: string;
	content:
		| AdvisorResultPayload
		| AdvisorRedactedResultPayload
		| AdvisorErrorPayload;
}

type ContentBlock =
	| TextBlock
	| ServerToolUseBlock
	| AdvisorToolResultBlock
	| { type: string; [key: string]: unknown };

interface UsageIteration {
	type: string;
	model?: string;
	input_tokens?: number;
	output_tokens?: number;
	cache_read_input_tokens?: number;
	cache_creation_input_tokens?: number;
}

interface Usage {
	input_tokens: number;
	output_tokens: number;
	iterations?: UsageIteration[];
}

interface MessagesResponse {
	id: string;
	type: string;
	role: string;
	content: ContentBlock[];
	model: string;
	stop_reason: string | null;
	usage: Usage;
}

interface CliArgs {
	questionFile: string | null;
	debug: boolean;
}

function parseArgs(argv: string[]): CliArgs {
	let questionFile: string | null = null;
	let debug = false;
	for (let i = 0; i < argv.length; i++) {
		const arg = argv[i];
		if (arg === "--question-file") {
			questionFile = argv[i + 1] ?? null;
			i++;
		} else if (arg === "--debug") {
			debug = true;
		}
	}
	return { questionFile, debug };
}

function die(code: number, message: string): never {
	console.error(message);
	process.exit(code);
}

function printUsage(): void {
	console.error("Usage: advisor-cli.ts --question-file <path> [--debug]");
	console.error(
		"Exit codes: 0 success, 1 usage error, 2 runtime failure (caller should fall back).",
	);
}

function isTextBlock(block: ContentBlock): block is TextBlock {
	return block.type === "text" && typeof (block as TextBlock).text === "string";
}

function isAdvisorResultBlock(
	block: ContentBlock,
): block is AdvisorToolResultBlock {
	return block.type === "advisor_tool_result";
}

async function main(): Promise<void> {
	const { questionFile, debug } = parseArgs(process.argv.slice(2));

	if (!questionFile) {
		printUsage();
		process.exit(1);
	}

	const apiKey = process.env.ANTHROPIC_API_KEY;
	if (!apiKey) {
		die(
			2,
			"advisor-cli: ANTHROPIC_API_KEY not set; caller should fall back to in-thread reasoning",
		);
	}

	let question: string;
	try {
		question = readFileSync(questionFile, "utf-8");
	} catch (err: unknown) {
		die(
			2,
			`advisor-cli: failed to read question file: ${err instanceof Error ? err.message : String(err)}`,
		);
	}

	const requestBody = {
		model: EXECUTOR_MODEL,
		max_tokens: MAX_TOKENS,
		system: SYSTEM_PROMPT,
		tools: [
			{
				type: "advisor_20260301",
				name: "advisor",
				model: ADVISOR_MODEL,
			},
		],
		messages: [
			{
				role: "user",
				content: question,
			},
		],
	};

	let response: Response;
	try {
		response = await fetch(API_URL, {
			method: "POST",
			headers: {
				"x-api-key": apiKey,
				"anthropic-version": ANTHROPIC_VERSION,
				"anthropic-beta": BETA_HEADER,
				"content-type": "application/json",
			},
			body: JSON.stringify(requestBody),
		});
	} catch (err: unknown) {
		die(
			2,
			`advisor-cli: network error: ${err instanceof Error ? err.message : String(err)}`,
		);
	}

	if (!response.ok) {
		const body = await response.text().catch(() => "<unreadable body>");
		die(2, `advisor-cli: HTTP ${response.status}: ${body}`);
	}

	let parsed: MessagesResponse;
	try {
		parsed = (await response.json()) as MessagesResponse;
	} catch (err: unknown) {
		die(
			2,
			`advisor-cli: failed to parse response JSON: ${err instanceof Error ? err.message : String(err)}`,
		);
	}

	if (
		parsed.stop_reason &&
		parsed.stop_reason !== "end_turn" &&
		parsed.stop_reason !== "stop_sequence"
	) {
		console.error(
			`advisor-cli: warning: stop_reason=${parsed.stop_reason} (response may be truncated)`,
		);
	}

	const textParts: string[] = [];
	for (const block of parsed.content) {
		if (isTextBlock(block)) {
			textParts.push(block.text);
		}
	}

	const finalText = textParts.join("\n\n").trim();
	if (finalText.length > 0) {
		console.log(finalText);
	} else {
		console.log("(advisor-cli: no text returned from executor)");
	}

	if (debug) {
		console.error("--- advisor-cli debug ---");
		console.error(`stop_reason: ${parsed.stop_reason}`);
		console.error(`executor model: ${parsed.model}`);
		console.error(
			`top-level usage: input=${parsed.usage.input_tokens} output=${parsed.usage.output_tokens}`,
		);
		console.error("usage.iterations:");
		for (const it of parsed.usage.iterations ?? []) {
			const modelStr = it.model ? ` model=${it.model}` : "";
			console.error(
				`  type=${it.type}${modelStr} in=${it.input_tokens ?? 0} out=${it.output_tokens ?? 0}`,
			);
		}
		let advisorCalls = 0;
		for (const block of parsed.content) {
			if (isAdvisorResultBlock(block)) advisorCalls++;
		}
		console.error(`advisor_tool_result blocks in content: ${advisorCalls}`);
	}
}

main().catch((err: unknown) => {
	console.error(err);
	process.exit(2);
});
