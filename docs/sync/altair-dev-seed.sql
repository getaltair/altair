-- Altair development seed dataset
-- Assumes baseline schema already exists.

-- Fixed UUIDs keep local testing deterministic.
-- Users
INSERT INTO users (id, email, display_name, timezone, is_active)
VALUES
  ('11111111-1111-1111-1111-111111111111', 'robert@example.com', 'Robert', 'America/Chicago', true),
  ('22222222-2222-2222-2222-222222222222', 'alex@example.com', 'Alex', 'America/Chicago', true)
ON CONFLICT (id) DO NOTHING;

-- Household
INSERT INTO households (id, owner_user_id, name, slug, description)
VALUES
  ('33333333-3333-3333-3333-333333333333', '11111111-1111-1111-1111-111111111111', 'Home', 'home', 'Primary shared household')
ON CONFLICT (id) DO NOTHING;

UPDATE users
SET default_household_id = '33333333-3333-3333-3333-333333333333'
WHERE id IN ('11111111-1111-1111-1111-111111111111', '22222222-2222-2222-2222-222222222222');

-- Memberships
INSERT INTO household_memberships (id, household_id, user_id, role, is_active)
VALUES
  ('44444444-4444-4444-4444-444444444441', '33333333-3333-3333-3333-333333333333', '11111111-1111-1111-1111-111111111111', 'owner', true),
  ('44444444-4444-4444-4444-444444444442', '33333333-3333-3333-3333-333333333333', '22222222-2222-2222-2222-222222222222', 'member', true)
ON CONFLICT (household_id, user_id) DO NOTHING;

-- Initiatives
INSERT INTO initiatives (id, owner_user_id, household_id, created_by_user_id, updated_by_user_id, title, slug, description, status)
VALUES
  ('55555555-5555-5555-5555-555555555551', '11111111-1111-1111-1111-111111111111', NULL, '11111111-1111-1111-1111-111111111111', '11111111-1111-1111-1111-111111111111', 'Altair Buildout', 'altair-buildout', 'Personal product/build initiative', 'active'),
  ('55555555-5555-5555-5555-555555555552', NULL, '33333333-3333-3333-3333-333333333333', '11111111-1111-1111-1111-111111111111', '11111111-1111-1111-1111-111111111111', 'Home Operations', 'home-ops', 'Shared household operations initiative', 'active')
ON CONFLICT (id) DO NOTHING;

-- Tags
INSERT INTO tags (id, owner_user_id, household_id, created_by_user_id, updated_by_user_id, name, color, description)
VALUES
  ('66666666-6666-6666-6666-666666666661', '11111111-1111-1111-1111-111111111111', NULL, '11111111-1111-1111-1111-111111111111', '11111111-1111-1111-1111-111111111111', 'lab', '#3b82f6', 'Personal lab work'),
  ('66666666-6666-6666-6666-666666666662', NULL, '33333333-3333-3333-3333-333333333333', '11111111-1111-1111-1111-111111111111', '11111111-1111-1111-1111-111111111111', 'household', '#22c55e', 'Shared household tag'),
  ('66666666-6666-6666-6666-666666666663', NULL, '33333333-3333-3333-3333-333333333333', '11111111-1111-1111-1111-111111111111', '11111111-1111-1111-1111-111111111111', 'maintenance', '#f59e0b', 'Maintenance-related')
ON CONFLICT (id) DO NOTHING;

-- Guidance
INSERT INTO guidance_epics (id, initiative_id, owner_user_id, household_id, created_by_user_id, updated_by_user_id, title, description, status, priority, sort_order)
VALUES
  ('77777777-7777-7777-7777-777777777771', '55555555-5555-5555-5555-555555555551', '11111111-1111-1111-1111-111111111111', NULL, '11111111-1111-1111-1111-111111111111', '11111111-1111-1111-1111-111111111111', 'Foundation', 'Core system foundations', 'active', 'high', 1)
ON CONFLICT (id) DO NOTHING;

INSERT INTO guidance_quests (id, initiative_id, epic_id, owner_user_id, household_id, assigned_user_id, created_by_user_id, updated_by_user_id, title, description, status, priority, energy_level, due_at, scheduled_for, completed_at, sort_order)
VALUES
  ('88888888-8888-8888-8888-888888888881', '55555555-5555-5555-5555-555555555551', '77777777-7777-7777-7777-777777777771', '11111111-1111-1111-1111-111111111111', NULL, '11111111-1111-1111-1111-111111111111', '11111111-1111-1111-1111-111111111111', '11111111-1111-1111-1111-111111111111', 'Define PowerSync streams', 'Create initial sync stream definitions', 'in_progress', 'high', 'medium', now() + interval '3 day', current_date, NULL, 1),
  ('88888888-8888-8888-8888-888888888882', '55555555-5555-5555-5555-555555555552', NULL, NULL, '33333333-3333-3333-3333-333333333333', '22222222-2222-2222-2222-222222222222', '11111111-1111-1111-1111-111111111111', '22222222-2222-2222-2222-222222222222', 'Take out the trash', 'Shared household chore', 'todo', 'medium', 'low', now() + interval '1 day', current_date, NULL, 1),
  ('88888888-8888-8888-8888-888888888883', '55555555-5555-5555-5555-555555555552', NULL, NULL, '33333333-3333-3333-3333-333333333333', '11111111-1111-1111-1111-111111111111', '11111111-1111-1111-1111-111111111111', '11111111-1111-1111-1111-111111111111', 'Replace HVAC filter', 'Monthly home maintenance', 'completed', 'medium', 'medium', now() - interval '2 day', current_date - 2, now() - interval '1 day', 2)
ON CONFLICT (id) DO NOTHING;

INSERT INTO guidance_routines (id, owner_user_id, household_id, initiative_id, created_by_user_id, updated_by_user_id, title, description, cadence_type, cadence_config, is_active, last_completed_at)
VALUES
  ('99999999-9999-9999-9999-999999999991', '11111111-1111-1111-1111-111111111111', NULL, '55555555-5555-5555-5555-555555555551', '11111111-1111-1111-1111-111111111111', '11111111-1111-1111-1111-111111111111', 'Daily planning', 'Morning planning routine', 'daily', '{"time":"07:30"}'::jsonb, true, now() - interval '1 day'),
  ('99999999-9999-9999-9999-999999999992', NULL, '33333333-3333-3333-3333-333333333333', '55555555-5555-5555-5555-555555555552', '11111111-1111-1111-1111-111111111111', '11111111-1111-1111-1111-111111111111', 'Trash night', 'Weekly trash routine', 'weekly', '{"day":"Thursday"}'::jsonb, true, now() - interval '7 day')
ON CONFLICT (id) DO NOTHING;

-- Knowledge
INSERT INTO knowledge_notes (id, owner_user_id, household_id, initiative_id, parent_note_id, created_by_user_id, updated_by_user_id, title, slug, note_type, content_markdown, content_text, is_pinned)
VALUES
  ('aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaa1', '11111111-1111-1111-1111-111111111111', NULL, '55555555-5555-5555-5555-555555555551', NULL, '11111111-1111-1111-1111-111111111111', '11111111-1111-1111-1111-111111111111', 'PowerSync Notes', 'powersync-notes', 'reference', '# PowerSync Notes

Need user, household, and initiative scopes.', 'PowerSync Notes Need user, household, and initiative scopes.', true),
  ('aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaa2', NULL, '33333333-3333-3333-3333-333333333333', '55555555-5555-5555-5555-555555555552', NULL, '11111111-1111-1111-1111-111111111111', '11111111-1111-1111-1111-111111111111', 'HVAC Maintenance', 'hvac-maintenance', 'guide', '# HVAC Maintenance

Use MERV 11 filters.', 'HVAC Maintenance Use MERV 11 filters.', false)
ON CONFLICT (id) DO NOTHING;

INSERT INTO knowledge_note_snapshots (id, note_id, created_by_user_id, snapshot_type, content_markdown, content_text, metadata_json)
VALUES
  ('bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbb1', 'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaa1', '11111111-1111-1111-1111-111111111111', 'manual', '# PowerSync Notes

Initial draft.', 'PowerSync Notes Initial draft.', '{"reason":"first snapshot"}'::jsonb)
ON CONFLICT (id) DO NOTHING;

-- Tracking taxonomy
INSERT INTO tracking_categories (id, owner_user_id, household_id, created_by_user_id, updated_by_user_id, name, description)
VALUES
  ('cccccccc-cccc-cccc-cccc-ccccccccccc1', NULL, '33333333-3333-3333-3333-333333333333', '11111111-1111-1111-1111-111111111111', '11111111-1111-1111-1111-111111111111', 'grocery', 'Household groceries'),
  ('cccccccc-cccc-cccc-cccc-ccccccccccc2', NULL, '33333333-3333-3333-3333-333333333333', '11111111-1111-1111-1111-111111111111', '11111111-1111-1111-1111-111111111111', 'maintenance_part', 'Home maintenance parts')
ON CONFLICT (id) DO NOTHING;

INSERT INTO tracking_locations (id, owner_user_id, household_id, parent_location_id, created_by_user_id, updated_by_user_id, name, location_type, description)
VALUES
  ('dddddddd-dddd-dddd-dddd-ddddddddddd1', NULL, '33333333-3333-3333-3333-333333333333', NULL, '11111111-1111-1111-1111-111111111111', '11111111-1111-1111-1111-111111111111', 'Kitchen Pantry', 'room', 'Primary pantry'),
  ('dddddddd-dddd-dddd-dddd-ddddddddddd2', NULL, '33333333-3333-3333-3333-333333333333', NULL, '11111111-1111-1111-1111-111111111111', '11111111-1111-1111-1111-111111111111', 'Hall Closet', 'storage', 'Hall storage closet')
ON CONFLICT (id) DO NOTHING;

INSERT INTO tracking_items (id, owner_user_id, household_id, initiative_id, location_id, category_id, created_by_user_id, updated_by_user_id, name, description, sku, barcode, quantity, unit, min_quantity, reorder_quantity, is_consumable, expires_at, last_seen_at, metadata_json)
VALUES
  ('eeeeeeee-eeee-eeee-eeee-eeeeeeeeeee1', NULL, '33333333-3333-3333-3333-333333333333', '55555555-5555-5555-5555-555555555552', 'dddddddd-dddd-dddd-dddd-ddddddddddd1', 'cccccccc-cccc-cccc-cccc-ccccccccccc1', '11111111-1111-1111-1111-111111111111', '11111111-1111-1111-1111-111111111111', 'Trash Bags', '13 gallon kitchen bags', 'TB-13', '0123456789012', 8, 'count', 2, 10, true, NULL, now(), '{"brand":"Generic"}'::jsonb),
  ('eeeeeeee-eeee-eeee-eeee-eeeeeeeeeee2', NULL, '33333333-3333-3333-3333-333333333333', '55555555-5555-5555-5555-555555555552', 'dddddddd-dddd-dddd-dddd-ddddddddddd2', 'cccccccc-cccc-cccc-cccc-ccccccccccc2', '11111111-1111-1111-1111-111111111111', '11111111-1111-1111-1111-111111111111', 'HVAC Filter 20x25x1', 'MERV 11 replacement filter', 'HVAC-20251', '0987654321098', 2, 'count', 1, 2, true, NULL, now(), '{"merv":"11"}'::jsonb),
  ('eeeeeeee-eeee-eeee-eeee-eeeeeeeeeee3', NULL, '33333333-3333-3333-3333-333333333333', '55555555-5555-5555-5555-555555555552', 'dddddddd-dddd-dddd-dddd-ddddddddddd1', 'cccccccc-cccc-cccc-cccc-ccccccccccc1', '11111111-1111-1111-1111-111111111111', '11111111-1111-1111-1111-111111111111', 'Milk', 'Whole milk gallon', 'MILK-1G', '1111111111111', 1, 'count', 1, 2, true, now() + interval '5 day', now(), '{"brand":"Store"}'::jsonb)
ON CONFLICT (id) DO NOTHING;

INSERT INTO tracking_item_events (id, item_id, performed_by_user_id, event_type, delta_quantity, from_location_id, to_location_id, notes, metadata_json, occurred_at)
VALUES
  ('ffffffff-ffff-ffff-ffff-fffffffffff1', 'eeeeeeee-eeee-eeee-eeee-eeeeeeeeeee1', '22222222-2222-2222-2222-222222222222', 'consume', -1, NULL, NULL, 'Used one bag for kitchen trash', '{"source":"manual"}'::jsonb, now() - interval '2 hour'),
  ('ffffffff-ffff-ffff-ffff-fffffffffff2', 'eeeeeeee-eeee-eeee-eeee-eeeeeeeeeee2', '11111111-1111-1111-1111-111111111111', 'maintenance', NULL, NULL, NULL, 'Installed new filter', '{"hvac_unit":"main"}'::jsonb, now() - interval '1 day'),
  ('ffffffff-ffff-ffff-ffff-fffffffffff3', 'eeeeeeee-eeee-eeee-eeee-eeeeeeeeeee3', '11111111-1111-1111-1111-111111111111', 'restock', 1, NULL, NULL, 'Bought one gallon', '{"store":"Local Market"}'::jsonb, now() - interval '3 day')
ON CONFLICT (id) DO NOTHING;

-- Shopping list
INSERT INTO tracking_shopping_lists (id, owner_user_id, household_id, created_by_user_id, updated_by_user_id, name, status)
VALUES
  ('12121212-1212-1212-1212-121212121212', NULL, '33333333-3333-3333-3333-333333333333', '11111111-1111-1111-1111-111111111111', '11111111-1111-1111-1111-111111111111', 'Weekly Groceries', 'active')
ON CONFLICT (id) DO NOTHING;

INSERT INTO tracking_shopping_list_items (id, shopping_list_id, item_id, created_by_user_id, updated_by_user_id, name, quantity, unit, is_completed, notes)
VALUES
  ('13131313-1313-1313-1313-131313131311', '12121212-1212-1212-1212-121212121212', 'eeeeeeee-eeee-eeee-eeee-eeeeeeeeeee3', '11111111-1111-1111-1111-111111111111', '11111111-1111-1111-1111-111111111111', 'Milk', 2, 'count', false, 'Need more before weekend'),
  ('13131313-1313-1313-1313-131313131312', '12121212-1212-1212-1212-121212121212', NULL, '22222222-2222-2222-2222-222222222222', '22222222-2222-2222-2222-222222222222', 'Paper Towels', 1, 'count', false, NULL)
ON CONFLICT (id) DO NOTHING;

-- Tag joins
INSERT INTO initiative_tags (initiative_id, tag_id) VALUES
  ('55555555-5555-5555-5555-555555555551', '66666666-6666-6666-6666-666666666661'),
  ('55555555-5555-5555-5555-555555555552', '66666666-6666-6666-6666-666666666662')
ON CONFLICT DO NOTHING;

INSERT INTO quest_tags (quest_id, tag_id) VALUES
  ('88888888-8888-8888-8888-888888888882', '66666666-6666-6666-6666-666666666662'),
  ('88888888-8888-8888-8888-888888888883', '66666666-6666-6666-6666-666666666663')
ON CONFLICT DO NOTHING;

INSERT INTO note_tags (note_id, tag_id) VALUES
  ('aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaa1', '66666666-6666-6666-6666-666666666661'),
  ('aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaa2', '66666666-6666-6666-6666-666666666663')
ON CONFLICT DO NOTHING;

INSERT INTO item_tags (item_id, tag_id) VALUES
  ('eeeeeeee-eeee-eeee-eeee-eeeeeeeeeee2', '66666666-6666-6666-6666-666666666663')
ON CONFLICT DO NOTHING;

-- Attachments
INSERT INTO attachments (id, owner_user_id, household_id, initiative_id, created_by_user_id, updated_by_user_id, storage_key, original_filename, mime_type, size_bytes, sha256, processing_status, metadata_json)
VALUES
  ('14141414-1414-1414-1414-141414141411', '11111111-1111-1111-1111-111111111111', NULL, '55555555-5555-5555-5555-555555555551', '11111111-1111-1111-1111-111111111111', '11111111-1111-1111-1111-111111111111', 'attachments/powersync-diagram.png', 'powersync-diagram.png', 'image/png', 204800, 'abc123', 'ready', '{"kind":"diagram"}'::jsonb),
  ('14141414-1414-1414-1414-141414141412', NULL, '33333333-3333-3333-3333-333333333333', '55555555-5555-5555-5555-555555555552', '11111111-1111-1111-1111-111111111111', '11111111-1111-1111-1111-111111111111', 'attachments/hvac-filter.jpg', 'hvac-filter.jpg', 'image/jpeg', 102400, 'def456', 'ready', '{"kind":"photo"}'::jsonb)
ON CONFLICT (id) DO NOTHING;

INSERT INTO note_attachments (note_id, attachment_id) VALUES
  ('aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaa1', '14141414-1414-1414-1414-141414141411'),
  ('aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaa2', '14141414-1414-1414-1414-141414141412')
ON CONFLICT DO NOTHING;

-- Semantic / cross-domain relations
INSERT INTO entity_relations (
  id, household_id, initiative_id, owner_user_id, created_by_user_id, updated_by_user_id,
  from_entity_type, from_entity_id, to_entity_type, to_entity_id,
  relation_type, source_type, status, confidence, evidence_json, created_by_process, last_confirmed_at
)
VALUES
  (
    '15151515-1515-1515-1515-151515151511',
    NULL,
    '55555555-5555-5555-5555-555555555551',
    '11111111-1111-1111-1111-111111111111',
    '11111111-1111-1111-1111-111111111111',
    '11111111-1111-1111-1111-111111111111',
    'knowledge_note',
    'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaa1',
    'initiative',
    '55555555-5555-5555-5555-555555555551',
    'supports',
    'user',
    'accepted',
    1.0,
    '{"reason":"planning note for initiative"}'::jsonb,
    NULL,
    now()
  ),
  (
    '15151515-1515-1515-1515-151515151512',
    '33333333-3333-3333-3333-333333333333',
    '55555555-5555-5555-5555-555555555552',
    NULL,
    '11111111-1111-1111-1111-111111111111',
    '11111111-1111-1111-1111-111111111111',
    'guidance_quest',
    '88888888-8888-8888-8888-888888888883',
    'tracking_item',
    'eeeeeeee-eeee-eeee-eeee-eeeeeeeeeee2',
    'requires',
    'user',
    'accepted',
    1.0,
    '{"reason":"filter replacement consumes this item"}'::jsonb,
    NULL,
    now()
  ),
  (
    '15151515-1515-1515-1515-151515151513',
    '33333333-3333-3333-3333-333333333333',
    '55555555-5555-5555-5555-555555555552',
    NULL,
    '11111111-1111-1111-1111-111111111111',
    '11111111-1111-1111-1111-111111111111',
    'knowledge_note',
    'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaa2',
    'tracking_item',
    'eeeeeeee-eeee-eeee-eeee-eeeeeeeeeee2',
    'references',
    'user',
    'accepted',
    1.0,
    '{"reason":"maintenance note references specific filter"}'::jsonb,
    NULL,
    now()
  ),
  (
    '15151515-1515-1515-1515-151515151514',
    '33333333-3333-3333-3333-333333333333',
    '55555555-5555-5555-5555-555555555552',
    NULL,
    '11111111-1111-1111-1111-111111111111',
    '11111111-1111-1111-1111-111111111111',
    'tracking_item',
    'eeeeeeee-eeee-eeee-eeee-eeeeeeeeeee1',
    'guidance_quest',
    '88888888-8888-8888-8888-888888888882',
    'related_to',
    'ai',
    'suggested',
    0.7300,
    '{"reason":"trash bags likely support trash chore"}'::jsonb,
    'seed_ai_demo',
    NULL
  ),
  (
    '15151515-1515-1515-1515-151515151515',
    '33333333-3333-3333-3333-333333333333',
    '55555555-5555-5555-5555-555555555552',
    NULL,
    '11111111-1111-1111-1111-111111111111',
    '11111111-1111-1111-1111-111111111111',
    'knowledge_note',
    'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaa2',
    'guidance_quest',
    '88888888-8888-8888-8888-888888888883',
    'supports',
    'user',
    'dismissed',
    1.0,
    '{"reason":"HVAC note supports filter replacement quest"}'::jsonb,
    NULL,
    NULL
  )
ON CONFLICT (id) DO NOTHING;
