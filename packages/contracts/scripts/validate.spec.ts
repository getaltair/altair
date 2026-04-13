import { test, expect } from 'bun:test';
import { readFileSync, writeFileSync } from 'node:fs';

const REPO_ROOT = '/home/rghamilton3/workspace/getaltair/altair';
const SCRIPT = `${REPO_ROOT}/packages/contracts/scripts/validate.ts`;
const ENTITY_TYPES_JSON = `${REPO_ROOT}/packages/contracts/entity-types.json`;
const KOTLIN_ENTITY_TYPE = `${REPO_ROOT}/apps/android/app/src/main/java/com/getaltair/altair/contracts/EntityType.kt`;

function runValidate() {
  return Bun.spawnSync(['bun', 'run', SCRIPT], { cwd: REPO_ROOT });
}

test('1: clean repo exits 0', () => {
  const proc = runValidate();
  expect(proc.exitCode).toBe(0);
});

test('2: extra entry in entity-types.json causes exit 1 and names the offending id', () => {
  const original = readFileSync(ENTITY_TYPES_JSON, 'utf8');
  try {
    const registry = JSON.parse(original);
    registry.values.push({ id: 'test_fake_type', description: 'Test' });
    writeFileSync(ENTITY_TYPES_JSON, JSON.stringify(registry, null, 2));

    const proc = runValidate();
    const output = proc.stdout.toString() + new TextDecoder().decode(proc.stderr);
    expect(proc.exitCode).toBe(1);
    expect(output).toContain('test_fake_type');
  } finally {
    writeFileSync(ENTITY_TYPES_JSON, original);
  }
});

test('3: after restoring entity-types.json, exits 0 again', () => {
  const proc = runValidate();
  expect(proc.exitCode).toBe(0);
});

test('4: removing USER entry from Kotlin EntityType.kt causes exit 1', () => {
  const original = readFileSync(KOTLIN_ENTITY_TYPE, 'utf8');
  try {
    // Remove the USER("user"), line
    const modified = original.replace(/^\s*USER\("user"\),\n/m, '');
    writeFileSync(KOTLIN_ENTITY_TYPE, modified);

    const proc = runValidate();
    expect(proc.exitCode).toBe(1);
  } finally {
    writeFileSync(KOTLIN_ENTITY_TYPE, original);
  }
});
