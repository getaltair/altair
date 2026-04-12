/**
 * Secret Detection Hook (UserPromptSubmit)
 *
 * Scans every prompt for patterns that look like leaked secrets.
 * Outputs a warning if found. Never blocks (always exits 0).
 */

import { readFileSync } from "node:fs";

interface PromptPayload {
	prompt: string;
}

const SECRET_PATTERNS: { name: string; pattern: RegExp }[] = [
	{ name: "Anthropic/OpenAI API key", pattern: /\bsk-[a-zA-Z0-9]{20,}\b/ },
	{ name: "GitHub PAT", pattern: /\bghp_[a-zA-Z0-9]{36,}\b/ },
	{ name: "AWS access key", pattern: /\bAKIA[A-Z0-9]{16}\b/ },
	{ name: "GitLab PAT", pattern: /\bglpat-[a-zA-Z0-9_-]{20,}\b/ },
	{ name: "Slack bot token", pattern: /\bxoxb-[a-zA-Z0-9-]+\b/ },
	{
		name: "Private key material",
		pattern: /-----BEGIN [A-Z ]+ PRIVATE KEY-----/,
	},
	{
		name: "Inline password",
		pattern: /password\s*[:=]\s*["'][^"']{8,}/i,
	},
];

try {
	const raw = readFileSync(0, "utf-8");
	const data = JSON.parse(raw) as PromptPayload;
	const prompt = data.prompt ?? "";

	const findings: string[] = [];
	for (const { name, pattern } of SECRET_PATTERNS) {
		if (pattern.test(prompt)) {
			findings.push(name);
		}
	}

	if (findings.length > 0) {
		console.log(
			[
				"SECRET DETECTION WARNING",
				"",
				"Potential secrets detected in your prompt:",
				...findings.map((f) => `  - ${f}`),
				"",
				"Review your prompt and remove any API keys, tokens, or passwords.",
				"If these are intentional (e.g., asking about key formats), ignore this warning.",
			].join("\n"),
		);
	}

	process.exit(0);
} catch {
	// Silent failure -- never block the user
	process.exit(0);
}
