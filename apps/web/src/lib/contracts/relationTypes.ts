// Source of truth: packages/contracts/relation-types.json
export const RelationType = {
  References: "references",
  Supports: "supports",
  Requires: "requires",
  RelatedTo: "related_to",
  DependsOn: "depends_on",
  Duplicates: "duplicates",
  SimilarTo: "similar_to",
  GeneratedFrom: "generated_from",
  NoteLink: "note_link",
} as const;

export type RelationTypeValue =
  (typeof RelationType)[keyof typeof RelationType];
