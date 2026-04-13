#!/usr/bin/env bun
/**
 * Validates that all language bindings match their source JSON registries.
 * Run: bun run packages/contracts/scripts/validate.ts
 * Exit 0 = all bindings match. Exit 1 = drift detected.
 */

import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { resolve } from 'node:path';

// Resolve repo root relative to this script's location (packages/contracts/scripts/).
const ROOT = resolve(fileURLToPath(new URL('../../..', import.meta.url)));

type Registry = { contracts_version: string; values: { id: string; description: string }[] };

function loadRegistry(relPath: string): string[] {
  const abs = resolve(ROOT, relPath);
  let reg: Registry;
  try {
    reg = JSON.parse(readFileSync(abs, 'utf8'));
  } catch (e) {
    throw new Error(`${relPath}: failed to parse — ${e}`);
  }
  if (!Array.isArray(reg.values)) {
    throw new Error(`${relPath}: missing or invalid "values" array`);
  }
  return reg.values.map((v) => v.id);
}

function diff(name: string, expected: string[], actual: string[]): string[] {
  const errors: string[] = [];
  const dupes = actual.filter((id, i) => actual.indexOf(id) !== i);
  if (dupes.length) errors.push(`${name}: duplicate values in binding: ${dupes.join(', ')}`);
  const missing = expected.filter((id) => !actual.includes(id));
  const extra = actual.filter((id) => !expected.includes(id));
  if (missing.length) errors.push(`${name}: missing from binding: ${missing.join(', ')}`);
  if (extra.length) errors.push(`${name}: extra in binding (not in registry): ${extra.join(', ')}`);
  return errors;
}

async function checkTypeScript(): Promise<string[]> {
  const errors: string[] = [];

  // Entity types
  const entityIds = loadRegistry('packages/contracts/entity-types.json');
  const entityFile = resolve(ROOT, 'apps/web/src/lib/contracts/entityTypes.ts');
  let entityMod: { EntityType: Record<string, string> };
  try {
    entityMod = await import(entityFile);
  } catch (e) {
    errors.push(`TS EntityType: failed to import ${entityFile} — ${e}`);
    return errors;
  }
  errors.push(...diff('TS EntityType', entityIds, Object.values(entityMod.EntityType)));

  // Relation types
  const relationIds = loadRegistry('packages/contracts/relation-types.json');
  const relationFile = resolve(ROOT, 'apps/web/src/lib/contracts/relationTypes.ts');
  let relationMod: { RelationType: Record<string, string> };
  try {
    relationMod = await import(relationFile);
  } catch (e) {
    errors.push(`TS RelationType: failed to import ${relationFile} — ${e}`);
    return errors;
  }
  errors.push(...diff('TS RelationType', relationIds, Object.values(relationMod.RelationType)));

  // Sync streams
  const streamIds = loadRegistry('packages/contracts/sync-streams.json');
  const streamFile = resolve(ROOT, 'apps/web/src/lib/contracts/syncStreams.ts');
  let streamMod: { SyncStream: Record<string, string> };
  try {
    streamMod = await import(streamFile);
  } catch (e) {
    errors.push(`TS SyncStream: failed to import ${streamFile} — ${e}`);
    return errors;
  }
  errors.push(...diff('TS SyncStream', streamIds, Object.values(streamMod.SyncStream)));

  return errors;
}

/**
 * Extracts serde rename values from a named enum block in Rust source text.
 * Scopes extraction to the enum body to avoid capturing field-level serde renames
 * from structs, which would cause false positives in drift detection.
 */
function extractRustEnumValues(contractsText: string, enumName: string): string[] {
  const enumBodyRegex = new RegExp(`enum\\s+${enumName}[^{]*\\{([^}]+)\\}`);
  const match = contractsText.match(enumBodyRegex);
  if (!match) return [];
  return [...match[1].matchAll(/serde\(rename\s*=\s*"([^"]+)"\)/g)].map((m) => m[1]);
}

function checkKotlin(): string[] {
  const errors: string[] = [];

  const entityIds = loadRegistry('packages/contracts/entity-types.json');
  const relationIds = loadRegistry('packages/contracts/relation-types.json');
  const streamIds = loadRegistry('packages/contracts/sync-streams.json');

  const extractKotlinValues = (filePath: string): string[] => {
    const text = readFileSync(resolve(ROOT, filePath), 'utf8');
    const values = [...text.matchAll(/"([^"]+)"\)/g)].map((m) => m[1]);
    if (values.length === 0) {
      errors.push(
        `Kotlin: no enum values extracted from ${filePath} — file may be empty or pattern did not match`
      );
    }
    return values;
  };

  const kotlinEntityValues = extractKotlinValues(
    'apps/android/app/src/main/java/com/getaltair/altair/contracts/EntityType.kt'
  );
  errors.push(...diff('Kotlin EntityType', entityIds, kotlinEntityValues));

  const kotlinRelationValues = extractKotlinValues(
    'apps/android/app/src/main/java/com/getaltair/altair/contracts/RelationType.kt'
  );
  errors.push(...diff('Kotlin RelationType', relationIds, kotlinRelationValues));

  const kotlinStreamValues = extractKotlinValues(
    'apps/android/app/src/main/java/com/getaltair/altair/contracts/SyncStream.kt'
  );
  errors.push(...diff('Kotlin SyncStream', streamIds, kotlinStreamValues));

  return errors;
}

function checkRust(): string[] {
  const errors: string[] = [];

  const entityIds = loadRegistry('packages/contracts/entity-types.json');
  const relationIds = loadRegistry('packages/contracts/relation-types.json');
  const streamIds = loadRegistry('packages/contracts/sync-streams.json');

  const contractsFile = resolve(ROOT, 'apps/server/server/src/contracts.rs');
  const contractsText = readFileSync(contractsFile, 'utf8');

  const rustEntityValues = extractRustEnumValues(contractsText, 'EntityType');
  const rustRelationValues = extractRustEnumValues(contractsText, 'RelationType');
  const rustStreamValues = extractRustEnumValues(contractsText, 'SyncStream');

  if (rustEntityValues.length === 0)
    errors.push(`Rust EntityType: enum block not found in ${contractsFile}`);
  if (rustRelationValues.length === 0)
    errors.push(`Rust RelationType: enum block not found in ${contractsFile}`);
  if (rustStreamValues.length === 0)
    errors.push(`Rust SyncStream: enum block not found in ${contractsFile}`);

  errors.push(...diff('Rust EntityType', entityIds, rustEntityValues));
  errors.push(...diff('Rust RelationType', relationIds, rustRelationValues));
  errors.push(...diff('Rust SyncStream', streamIds, rustStreamValues));

  return errors;
}

async function main() {
  console.log('Validating contract bindings against JSON registries...\n');

  const tsErrors = await checkTypeScript();
  const kotlinErrors = checkKotlin();
  const rustErrors = checkRust();

  const allErrors = [...tsErrors, ...kotlinErrors, ...rustErrors];

  if (allErrors.length === 0) {
    console.log('✓ All bindings match their registries.');
    process.exit(0);
  } else {
    console.error('✗ Binding drift detected:\n');
    for (const err of allErrors) {
      console.error(`  - ${err}`);
    }
    console.error(
      `\n${allErrors.length} issue(s) found. Update the binding files to match the JSON registries.`
    );
    process.exit(1);
  }
}

main().catch((err) => {
  console.error('validate.ts: unexpected error:', err);
  process.exit(1);
});
