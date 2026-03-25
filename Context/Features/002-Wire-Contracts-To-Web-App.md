# Quick Task 002: Wire Shared Contracts to Web App

## Understanding

Phase 1 (Registry & Schemas) and Phase 2 (Code Generation & Tests) are complete. The contracts package exists at `packages/contracts/` with generated TypeScript bindings and validation tests passing. The next logical step is to wire the contracts into at least one consuming platform to validate the end-to-end flow and enable usage.

Starting with the web app (SvelteKit/TypeScript) because:
1. It's already a TypeScript project with workspace support
2. The contracts package is already in the root workspace
3. Simple import path configuration change in svelte.config.js

## In Scope
- Update web app svelte.config.js to resolve `@altair/contracts` from workspace
- Create a simple example in web app that imports and uses contract constants
- Verify the import works and constants are usable
- Update web app package.json to include contracts as dependency (or rely on workspace)

## Out of Scope
- Full integration of contracts into all app features
- Adding Android/Rust integrations (those would be separate quick tasks)
- Setting up CI workflows for contracts (Phase 3 of original plan)
- Database schema updates using contracts

## Implementation

1. **Update svelte.config.js** to configure the alias for `@altair/contracts`:
   ```js
   const config = {
     kit: {
       alias: {
         '@altair/contracts': 'packages/contracts/src/typescript/index.ts',
       },
     },
   }
   }
   ```

2. **Create example usage** in `apps/web/src/lib/examples/contracts-demo.ts`:
   - Import `ENTITY_TYPES`, `SYNC_STREAMS`, etc. from `@altair/contracts`
   - Create simple constants using the imported values
   - Demonstrate using `EntityRef` and `RelationRecord` DTOs

3. **Test the import** by running TypeScript check:
   - Verify no type errors
   - Confirm constants can be imported and used

4. **Update documentation** by adding a note to web app README about contracts usage

## Tests & Docs

- **Tests:** None needed — this is a simple import verification task. TypeScript type checking will validate the import path works.
- **Docs:** Update `apps/web/README.md` with section "Shared Contracts" explaining how to import and use contract constants in web app code.
