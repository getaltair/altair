-- migrate:up
-- NOTE: This migration depends on guidance_quests (000013), knowledge_notes (000017),
-- tracking_items (000021), and initiatives (000007). In linear migration execution,
-- this file must run AFTER all referenced tables exist. Reorder if applying sequentially.

CREATE TABLE quest_tags (
  quest_id   UUID        NOT NULL,
  tag_id     UUID        NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

  PRIMARY KEY (quest_id, tag_id),
  CONSTRAINT fk_quest_tags_quests FOREIGN KEY (quest_id) REFERENCES guidance_quests(id) ON DELETE CASCADE,
  CONSTRAINT fk_quest_tags_tags   FOREIGN KEY (tag_id)   REFERENCES tags(id)            ON DELETE CASCADE
);

CREATE TABLE note_tags (
  note_id    UUID        NOT NULL,
  tag_id     UUID        NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

  PRIMARY KEY (note_id, tag_id),
  CONSTRAINT fk_note_tags_notes FOREIGN KEY (note_id) REFERENCES knowledge_notes(id) ON DELETE CASCADE,
  CONSTRAINT fk_note_tags_tags  FOREIGN KEY (tag_id)  REFERENCES tags(id)            ON DELETE CASCADE
);

CREATE TABLE item_tags (
  item_id    UUID        NOT NULL,
  tag_id     UUID        NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

  PRIMARY KEY (item_id, tag_id),
  CONSTRAINT fk_item_tags_items FOREIGN KEY (item_id) REFERENCES tracking_items(id) ON DELETE CASCADE,
  CONSTRAINT fk_item_tags_tags  FOREIGN KEY (tag_id)  REFERENCES tags(id)           ON DELETE CASCADE
);

CREATE TABLE initiative_tags (
  initiative_id UUID        NOT NULL,
  tag_id        UUID        NOT NULL,
  created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),

  PRIMARY KEY (initiative_id, tag_id),
  CONSTRAINT fk_initiative_tags_initiatives FOREIGN KEY (initiative_id) REFERENCES initiatives(id) ON DELETE CASCADE,
  CONSTRAINT fk_initiative_tags_tags        FOREIGN KEY (tag_id)        REFERENCES tags(id)        ON DELETE CASCADE
);
