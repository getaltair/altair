# Altair Contracts Tooling Bundle

This bundle adds two things to the shared-contracts setup:

- **code generation** from registry JSON
- **validation tests** to catch drift

## Included

- `scripts/generate_contracts.py`
- `tests/test_registry_shapes.py`
- `tests/test_generated_typescript.py`
- `tests/test_generated_kotlin.py`
- `tests/test_generated_rust.py`
- `examples/package.json`
- `examples/Cargo.toml`
- `examples/build.gradle.kts`
- `README.md`

## Recommended usage

1. Put your canonical registries under `packages/contracts/registry/`
2. Put generated outputs under `packages/contracts/generated/`
3. Run the generator whenever registry files change
4. Run the tests in CI

## Suggested CI rule

Fail the build if:
- generated files are stale
- registry values are invalid
- language bindings do not contain the canonical values

That saves you from future string soup archaeology.
