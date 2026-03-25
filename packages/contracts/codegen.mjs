import { readFileSync, writeFileSync, mkdirSync } from "fs";
import { join, dirname } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const registryDir = join(__dirname, "registry");
const genDir = join(__dirname, "generated");

// ─── Helpers ────────────────────────────────────────────────

function readRegistry(filename) {
  return JSON.parse(readFileSync(join(registryDir, filename), "utf-8"));
}

function writeGen(subdir, filename, content) {
  const dir = join(genDir, subdir);
  mkdirSync(dir, { recursive: true });
  writeFileSync(join(dir, filename), content, "utf-8");
  console.log(`  wrote ${subdir}/${filename}`);
}

function toScreamingSnake(s) {
  return s.toUpperCase();
}

function toPascalCase(s) {
  return s
    .split("_")
    .map((w) => w.charAt(0).toUpperCase() + w.slice(1))
    .join("");
}

// ─── Entity Types ───────────────────────────────────────────

function genEntityTypes() {
  const reg = readRegistry("entity-types.json");
  const allTypes = Object.values(reg.types).flat();

  // TypeScript
  const tsLines = allTypes.map((t) => `  ${toScreamingSnake(t)} = "${t}",`);
  writeGen(
    "typescript",
    "entityTypes.ts",
    `// AUTO-GENERATED from registry/entity-types.json — do not edit\n\nexport enum EntityType {\n${tsLines.join("\n")}\n}\n\nexport const ALL_ENTITY_TYPES: readonly string[] = Object.values(EntityType);\n`
  );

  // Kotlin
  const ktEntries = allTypes.map(
    (t) => `    ${toScreamingSnake(t)}("${t}"),`
  );
  // Fix last entry: replace trailing comma with semicolon
  ktEntries[ktEntries.length - 1] = ktEntries[ktEntries.length - 1].replace(
    /,$/,
    ";"
  );
  writeGen(
    "kotlin",
    "EntityType.kt",
    `// AUTO-GENERATED from registry/entity-types.json — do not edit\npackage com.altair.contracts\n\nenum class EntityType(val value: String) {\n${ktEntries.join("\n")}\n\n    companion object {\n        fun fromValue(value: String): EntityType =\n            entries.first { it.value == value }\n    }\n}\n`
  );

  // Rust
  const rsVariants = allTypes.map(
    (t) =>
      `    #[serde(rename = "${t}")]\n    ${toPascalCase(t)},`
  );
  writeGen(
    "rust",
    "entity_type.rs",
    `// AUTO-GENERATED from registry/entity-types.json — do not edit\nuse serde::{Deserialize, Serialize};\n\n#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]\npub enum EntityType {\n${rsVariants.join("\n")}\n}\n\nimpl EntityType {\n    pub fn as_str(&self) -> &'static str {\n        match self {\n${allTypes.map((t) => `            Self::${toPascalCase(t)} => "${t}",`).join("\n")}\n        }\n    }\n}\n`
  );
}

// ─── Simple String Enum Generator ───────────────────────────

function genSimpleEnum(registryFile, enumName, tsFile, ktFile, rsFile) {
  const reg = readRegistry(registryFile);
  const types = reg.types;

  // TypeScript
  const tsLines = types.map((t) => `  ${toScreamingSnake(t)} = "${t}",`);
  writeGen(
    "typescript",
    tsFile,
    `// AUTO-GENERATED from registry/${registryFile} — do not edit\n\nexport enum ${enumName} {\n${tsLines.join("\n")}\n}\n`
  );

  // Kotlin
  const ktEntries = types.map(
    (t) => `    ${toScreamingSnake(t)}("${t}"),`
  );
  ktEntries[ktEntries.length - 1] = ktEntries[ktEntries.length - 1].replace(
    /,$/,
    ";"
  );
  writeGen(
    "kotlin",
    ktFile,
    `// AUTO-GENERATED from registry/${registryFile} — do not edit\npackage com.altair.contracts\n\nenum class ${enumName}(val value: String) {\n${ktEntries.join("\n")}\n\n    companion object {\n        fun fromValue(value: String): ${enumName} =\n            entries.first { it.value == value }\n    }\n}\n`
  );

  // Rust
  const rsVariants = types.map(
    (t) =>
      `    #[serde(rename = "${t}")]\n    ${toPascalCase(t)},`
  );
  writeGen(
    "rust",
    rsFile,
    `// AUTO-GENERATED from registry/${registryFile} — do not edit\nuse serde::{Deserialize, Serialize};\n\n#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]\npub enum ${enumName} {\n${rsVariants.join("\n")}\n}\n\nimpl ${enumName} {\n    pub fn as_str(&self) -> &'static str {\n        match self {\n${types.map((t) => `            Self::${toPascalCase(t)} => "${t}",`).join("\n")}\n        }\n    }\n}\n`
  );
}

// ─── Sync Streams ───────────────────────────────────────────

function genSyncStreams() {
  const reg = readRegistry("sync-streams.json");
  const all = [...reg.auto_subscribed, ...reg.on_demand];

  // TypeScript
  const tsLines = all.map((t) => `  ${toScreamingSnake(t)} = "${t}",`);
  const tsAuto = reg.auto_subscribed
    .map((t) => `  SyncStream.${toScreamingSnake(t)},`)
    .join("\n");
  const tsOnDemand = reg.on_demand
    .map((t) => `  SyncStream.${toScreamingSnake(t)},`)
    .join("\n");
  writeGen(
    "typescript",
    "syncStreams.ts",
    `// AUTO-GENERATED from registry/sync-streams.json — do not edit\n\nexport enum SyncStream {\n${tsLines.join("\n")}\n}\n\nexport const AUTO_SUBSCRIBED_STREAMS: readonly SyncStream[] = [\n${tsAuto}\n];\n\nexport const ON_DEMAND_STREAMS: readonly SyncStream[] = [\n${tsOnDemand}\n];\n`
  );

  // Kotlin
  const ktEntries = all.map(
    (t) => `    ${toScreamingSnake(t)}("${t}"),`
  );
  ktEntries[ktEntries.length - 1] = ktEntries[ktEntries.length - 1].replace(
    /,$/,
    ";"
  );
  writeGen(
    "kotlin",
    "SyncStream.kt",
    `// AUTO-GENERATED from registry/sync-streams.json — do not edit\npackage com.altair.contracts\n\nenum class SyncStream(val value: String) {\n${ktEntries.join("\n")}\n\n    companion object {\n        val AUTO_SUBSCRIBED = listOf(\n${reg.auto_subscribed.map((t) => `            ${toScreamingSnake(t)},`).join("\n")}\n        )\n        val ON_DEMAND = listOf(\n${reg.on_demand.map((t) => `            ${toScreamingSnake(t)},`).join("\n")}\n        )\n    }\n}\n`
  );

  // Rust
  const rsVariants = all.map(
    (t) =>
      `    #[serde(rename = "${t}")]\n    ${toPascalCase(t)},`
  );
  writeGen(
    "rust",
    "sync_stream.rs",
    `// AUTO-GENERATED from registry/sync-streams.json — do not edit\nuse serde::{Deserialize, Serialize};\n\n#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]\npub enum SyncStream {\n${rsVariants.join("\n")}\n}\n\nimpl SyncStream {\n    pub fn as_str(&self) -> &'static str {\n        match self {\n${all.map((t) => `            Self::${toPascalCase(t)} => "${t}",`).join("\n")}\n        }\n    }\n\n    pub fn is_auto_subscribed(&self) -> bool {\n        matches!(self, ${reg.auto_subscribed.map((t) => `Self::${toPascalCase(t)}`).join(" | ")})\n    }\n}\n`
  );
}

// ─── TypeScript barrel ──────────────────────────────────────

function genTsIndex() {
  writeGen(
    "typescript",
    "index.ts",
    `// AUTO-GENERATED — do not edit\nexport { EntityType, ALL_ENTITY_TYPES } from "./entityTypes.js";\nexport { RelationType } from "./relationTypes.js";\nexport { RelationSource } from "./relationSources.js";\nexport { RelationStatus } from "./relationStatuses.js";\nexport { AttachmentState } from "./attachmentStates.js";\nexport { SyncStream, AUTO_SUBSCRIBED_STREAMS, ON_DEMAND_STREAMS } from "./syncStreams.js";\n`
  );
}

// ─── Rust mod.rs ────────────────────────────────────────────

function genRsMod() {
  writeGen(
    "rust",
    "mod.rs",
    `// AUTO-GENERATED — do not edit\npub mod entity_type;\npub mod relation_type;\npub mod relation_source;\npub mod relation_status;\npub mod attachment_state;\npub mod sync_stream;\n\npub use entity_type::EntityType;\npub use relation_type::RelationType;\npub use relation_source::RelationSource;\npub use relation_status::RelationStatus;\npub use attachment_state::AttachmentState;\npub use sync_stream::SyncStream;\n`
  );
}

// ─── Run ────────────────────────────────────────────────────

console.log("Generating contracts...");
genEntityTypes();
genSimpleEnum(
  "relation-types.json",
  "RelationType",
  "relationTypes.ts",
  "RelationType.kt",
  "relation_type.rs"
);
genSimpleEnum(
  "relation-sources.json",
  "RelationSource",
  "relationSources.ts",
  "RelationSource.kt",
  "relation_source.rs"
);
genSimpleEnum(
  "relation-statuses.json",
  "RelationStatus",
  "relationStatuses.ts",
  "RelationStatus.kt",
  "relation_status.rs"
);
genSimpleEnum(
  "attachment-states.json",
  "AttachmentState",
  "attachmentStates.ts",
  "AttachmentState.kt",
  "attachment_state.rs"
);
genSyncStreams();
genTsIndex();
genRsMod();
console.log("Done.");
