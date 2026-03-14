-- Migration: 0013_align_relation_enums
-- Description: Align relation enums with contracts package
-- Note: Complete enum replacement - no data migration (table must be empty)

-- Safety check: ensure entity_relations table is empty before proceeding
-- This prevents silent data loss from CASCADE operations
DO $$
BEGIN
	IF EXISTS (
		SELECT FROM information_schema.tables
		WHERE table_schema = 'public'
		AND table_name = 'entity_relations'
	) THEN
		IF EXISTS (SELECT 1 FROM entity_relations LIMIT 1) THEN
			RAISE EXCEPTION 'entity_relations table must be empty for this migration. Found existing data.';
		END IF;
	END IF;
END $$;

-- Drop existing enum types
-- These are complete replacements, not extensions
-- DROP TYPE IF EXISTS entity_type CASCADE;
-- DROP TYPE IF EXISTS relation_type CASCADE;
-- DROP TYPE IF EXISTS source_type CASCADE;
-- DROP TYPE IF EXISTS relation_status CASCADE;

-- Create new entity_type enum (18 values from contracts) with v2 suffix to avoid name conflict
-- Aligned with packages/contracts/registry/entity-types.json
CREATE TYPE entity_type_v2 AS enum (
	'user',
	'household',
	'initiative',
	'tag',
	'attachment',
	'guidance_epic',
	'guidance_quest',
	'guidance_routine',
	'guidance_focus_session',
	'guidance_daily_checkin',
	'knowledge_note',
	'knowledge_note_snapshot',
	'tracking_location',
	'tracking_category',
	'tracking_item',
	'tracking_shopping_list',
	'tracking_shopping_list_item'
);

-- Create new relation_type enum (8 values from contracts) with v2 suffix
-- Aligned with packages/contracts/registry/relation-types.json
CREATE TYPE relation_type_v2 AS enum (
	'references',
	'supports',
	'requires',
	'related_to',
	'depends_on',
	'duplicates',
	'similar_to',
	'generated_from'
);

-- Create new source_type enum (6 values from contracts) with v2 suffix
-- Note: 'manual' replaced with 'user'
CREATE TYPE source_type_v2 AS enum (
	'user',
	'ai',
	'import',
	'rule',
	'migration',
	'system'
);

-- Create new relation_status enum (5 values from contracts) with v2 suffix
-- Note: 'active' replaced with 'suggested'
CREATE TYPE relation_status_v2 AS enum (
	'accepted',
	'suggested',
	'dismissed',
	'rejected',
	'expired'
);

-- Alter entity_relations table to use new v2 enum types
-- Only proceed if table exists
DO $$
BEGIN
	IF EXISTS (
		SELECT FROM information_schema.tables
		WHERE table_schema = 'public'
		AND table_name = 'entity_relations'
	) THEN
		-- Alter columns to use new v2 enum types
		ALTER TABLE entity_relations
		ALTER COLUMN from_entity_type TYPE entity_type_v2
		USING from_entity_type::text::entity_type_v2;
		
		ALTER TABLE entity_relations
		ALTER COLUMN to_entity_type TYPE entity_type_v2
		USING to_entity_type::text::entity_type_v2;
		
		ALTER TABLE entity_relations
		ALTER COLUMN relation_type TYPE relation_type_v2
		USING relation_type::text::relation_type_v2;
		
		ALTER TABLE entity_relations
		ALTER COLUMN source_type TYPE source_type_v2
		USING source_type::text::source_type_v2;
		
		ALTER TABLE entity_relations
		ALTER COLUMN status TYPE relation_status_v2
		USING status::text::relation_status_v2;
	
	-- Update column defaults to new v2 enum values
		ALTER TABLE entity_relations
		ALTER COLUMN source_type SET DEFAULT 'user';
	
		ALTER TABLE entity_relations
		ALTER COLUMN status SET DEFAULT 'suggested';
	END IF;
END $$;

-- Recreate indexes that referenced old enum values
DROP INDEX IF EXISTS entity_relations_status_idx;
DROP INDEX IF EXISTS entity_relations_source_type_idx;
DROP INDEX IF EXISTS entity_relations_confidence_idx;

-- Recreate with new v2 enum values
CREATE INDEX entity_relations_status_idx_v2 ON entity_relations (status) WHERE status = 'suggested';
CREATE INDEX entity_relations_source_type_idx_v2 ON entity_relations (source_type) WHERE source_type = 'user';
CREATE INDEX entity_relations_confidence_idx_v2 ON entity_relations (confidence) WHERE source_type = 'ai';
