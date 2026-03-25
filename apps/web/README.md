# sv

Everything you need to build a Svelte project, powered by [`sv`](https://github.com/sveltejs/cli).

## Shared Contracts

This application uses canonical identifiers and schemas from the `@altair/contracts` package. The contracts package provides:

- **Entity types** — Canonical identifiers (user, household, initiative, etc.)
- **Relation types** — Types of relationships (references, supports, requires, etc.)
- **Sync streams** — PowerSync stream names (my_profile, my_memberships, etc.)
- **DTOs** — Shared data transfer objects (EntityRef, RelationRecord)

### Importing Contracts

```typescript
import { ENTITY_TYPES, RELATION_TYPES, SYNC_STREAMS } from '@altair/contracts';
import type {
	EntityRef,
	RelationRecord,
	EntityTypesValue,
	RelationTypesValue
} from '@altair/contracts';
```

### Using Contracts

```typescript
// Create an entity reference
const entityRef: EntityRef = {
	entityType: ENTITY_TYPES.HOUSEHOLD,
	entityId: 'household-123'
};

// Create a relation record
const relation: RelationRecord = {
	id: 'relation-abc',
	from: entityRef,
	to: {
		entityType: ENTITY_TYPES.USER,
		entityId: 'user-456'
	},
	relationType: RELATION_TYPES.RELATED_TO,
	sourceType: RELATION_SOURCE_TYPES.USER,
	status: RELATION_STATUS_TYPES.ACCEPTED,
	confidence: 0.9,
	evidence: {},
	createdAt: new Date().toISOString(),
	updatedAt: new Date().toISOString()
};
```

For more examples, see `src/lib/examples/contracts-demo.ts`.

## Creating a project

If you're seeing this, you've probably already done this step. Congrats!

```sh
# create a new project
npx sv create my-app
```

To recreate this project with the same configuration:

```sh
# recreate this project
bun x sv@0.12.6 create --template minimal --types ts --add prettier eslint vitest="usages:unit,component" playwright tailwindcss="plugins:typography,forms" sveltekit-adapter="adapter:node" devtools-json drizzle="database:postgresql+postgresql:postgres.js+docker:yes" better-auth="demo:none" mcp="ide:claude-code,opencode+setup:remote" paraglide="languageTags:en+demo:no" --install bun web
```

## Developing

Once you've created a project and installed dependencies with `npm install` (or `pnpm install` or `yarn`), start a development server:

```sh
npm run dev

# or start the server and open the app in a new browser tab
npm run dev -- --open
```

## Building

To create a production version of your app:

```sh
npm run build
```

You can preview the production build with `npm run preview`.

> To deploy your app, you may need to install an [adapter](https://svelte.dev/docs/kit/adapters) for your target environment.
