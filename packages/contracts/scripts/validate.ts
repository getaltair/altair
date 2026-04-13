#!/usr/bin/env bun
/**
 * Validates that all language bindings match their source JSON registries.
 * Run: bun run packages/contracts/scripts/validate.ts
 * Exit 0 = all bindings match. Exit 1 = drift detected.
 */

import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

const ROOT = resolve(import.meta.dir, '../../..');

type Registry = { contracts_version: string; values: { id: string; description: string }[] };

function loadRegistry(relPath: string): string[] {
  const abs = resolve(ROOT, relPath);
  const reg: Registry = JSON.parse(readFileSync(abs, 'utf8'));
  return reg.values.map((v) => v.id);
}

function diff(name: string, expected: string[], actual: string[]): string[] {
  const errors: string[] = [];
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
  const { EntityType } = await import(resolve(ROOT, 'apps/web/src/lib/contracts/entityTypes.ts'));
  const tsEntityValues = Object.values(EntityType) as string[];
  errors.push(...diff('TS EntityType', entityIds, tsEntityValues));

  // Relation types
  const relationIds = loadRegistry('packages/contracts/relation-types.json');
  const { RelationType } = await import(resolve(ROOT, 'apps/web/src/lib/contracts/relationTypes.ts'));
  const tsRelationValues = Object.values(RelationType) as string[];
  errors.push(...diff('TS RelationType', relationIds, tsRelationValues));

  // Sync streams
  const streamIds = loadRegistry('packages/contracts/sync-streams.json');
  const { SyncStream } = await import(resolve(ROOT, 'apps/web/src/lib/contracts/syncStreams.ts'));
  const tsStreamValues = Object.values(SyncStream) as string[];
  errors.push(...diff('TS SyncStream', streamIds, tsStreamValues));

  return errors;
}

function checkKotlin(): string[] {
  const errors: string[] = [];

  const entityIds = loadRegistry('packages/contracts/entity-types.json');
  const relationIds = loadRegistry('packages/contracts/relation-types.json');
  const streamIds = loadRegistry('packages/contracts/sync-streams.json');

  const extractKotlinValues = (filePath: string): string[] => {
    const text = readFileSync(resolve(ROOT, filePath), 'utf8');
    const matches = [...text.matchAll(/"([^"]+)"\)/g)].map((m) => m[1]);
    return matches;
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

  const contractsText = readFileSync(
    resolve(ROOT, 'apps/server/server/src/contracts.rs'),
    'utf8'
  );

  // Extract all serde rename values: #[serde(rename = "value")]
  const allRenames = [...contractsText.matchAll(/serde\(rename\s*=\s*"([^"]+)"\)/g)].map(
    (m) => m[1]
  );

  // Partition by known registry: entity types come first, then relations, then streams
  // Use registry sets to categorize
  const entitySet = new Set(entityIds);
  const relationSet = new Set(relationIds);
  const streamSet = new Set(streamIds);

  const rustEntityValues = allRenames.filter((v) => entitySet.has(v) || (!relationSet.has(v) && !streamSet.has(v) && entityIds.includes(v)));
  const rustRelationValues = allRenames.filter((v) => relationSet.has(v));
  const rustStreamValues = allRenames.filter((v) => streamSet.has(v));

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
    console.error(`\n${allErrors.length} issue(s) found. Update the binding files to match the JSON registries.`);
    process.exit(1);
  }
}

main().catch((err) => {
  console.error('validate.ts: unexpected error:', err);
  process.exit(1);
});
