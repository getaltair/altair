
# Altair Schema Design Specification (Condensed)

Domains:
- Core / Shared
- Guidance
- Knowledge
- Tracking
- entity_relations

Primary DB: PostgreSQL
Sync Layer: PowerSync
Client DB: SQLite

## Core Entities
users
households
household_memberships
initiatives
tags
attachments
entity_relations

## Guidance
guidance_epics
guidance_quests
guidance_routines
guidance_focus_sessions
guidance_daily_checkins

## Knowledge
knowledge_notes
knowledge_note_snapshots

## Tracking
tracking_locations
tracking_categories
tracking_items
tracking_item_events
tracking_shopping_lists
tracking_shopping_list_items

## Relationship Model

entity_relations:
- from_entity_type
- from_entity_id
- to_entity_type
- to_entity_id
- relation_type
- confidence
- source_type

Allows cross‑domain graph queries without a graph database.

## Sync Strategy

PowerSync replication scopes:

user scope
household scope
initiative scope

Large event tables (snapshots, item_events) should replicate selectively.
