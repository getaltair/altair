# Tasks API

**Status:** 📝 TODO - Phase 1 (Q4 2025 - Q1 2026)  
**Priority:** High

---

## TODO: Contents

This document should cover:

### 1. Tasks Resource Overview
- Task object structure
- Fields and data types
- Required vs optional fields

### 2. Endpoints

**GET /api/v1/tasks**
- List all tasks for user
- Filtering (status, project, priority)
- Pagination
- Sorting

**POST /api/v1/tasks**
- Create new task
- Standard creation
- Quick capture endpoint

**GET /api/v1/tasks/{id}**
- Get single task
- Include relationships (project, time entries)

**PUT /api/v1/tasks/{id}**
- Update task
- Partial updates (PATCH behavior)

**DELETE /api/v1/tasks/{id}**
- Soft delete task
- Restore from trash

**POST /api/v1/tasks/{id}/start**
- Start time tracking

**POST /api/v1/tasks/{id}/stop**
- Stop time tracking

**POST /api/v1/tasks/{id}/breakdown**
- AI-powered task breakdown (Phase 2)

### 3. Task Object
```json
{
  "id": "uuid",
  "user_id": "uuid",
  "project_id": "uuid",
  "title": "string",
  "description": "string",
  "status": "enum",
  "priority": "enum",
  "estimated_duration": "integer",
  "created_at": "timestamp",
  "updated_at": "timestamp"
}
```

### 4. Examples
- Create quick capture task
- List tasks with filters
- Update task status
- Start/stop time tracking

---

**Write this alongside:** Backend task service implementation  
**Related:** FEATURES.md task management section
