# Altair PowerSync Bundle

Included files:

- `altair-powersync-sync-spec.md`
- `powersync-streams.example.yaml`
- `altair-entity-type-registry.md`
- `altair-dev-seed.sql`

Recommended order:

1. apply baseline schema
2. load `altair-dev-seed.sql`
3. configure PowerSync with `powersync-streams.example.yaml`
4. test:
   - profile + memberships auto-sync
   - household inventory sync
   - shared quest completion propagation
   - on-demand note detail
   - on-demand item history
