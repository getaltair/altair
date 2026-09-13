---
type: impact-assessment
context: altair-v0.1-greenfield
---

# Impact Assessment

Altair has no application source yet, so this release changes no existing code,
API, schema, or persisted user data. Blast radius is represented by the epic
dependency graph rather than existing callers.

Shared contracts introduced by e01s03 affect every domain editor. Relationship
identity from e05 affects project/task associations, archive races, recall, and
export. Attachment metadata from e07 extends search only after e06 lexical
search exists. Recall depends on all domain records and relationships but owns
no mutations. e08 verifies integrated behavior without introducing another
domain model.

Each story must run impact assessment again before changing a shared module once
source exists. Database changes require explicit SQLx migrations, and persisted
contract changes require migration and restore coverage.
