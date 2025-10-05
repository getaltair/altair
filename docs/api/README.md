# Altair API Documentation

**Status:** 📝 TODO - To be written during Phase 1 (Q4 2025 - Q1 2026)
**Priority:** Medium-High
**Target Audience:** Developers integrating with Altair API

---

## TODO: What This Documentation Should Contain

### Purpose
Complete API reference for developers building integrations, plugins, or self-hosting Altair.

### Required Sections

1. **API Overview**
   - RESTful architecture
   - Base URL
   - Versioning strategy
   - Rate limiting
   - Response formats

2. **Authentication**
   - JWT token authentication
   - Obtaining tokens
   - Token refresh
   - Security best practices

3. **Core Resources**
   - Tasks API
   - Projects API
   - Time Entries API
   - Users API
   - Notes/Documentation API

4. **Endpoints Reference**
   - Complete endpoint listing
   - Request/response examples
   - Error codes
   - Query parameters

5. **WebSocket API** (Future - Phase 3)
   - Real-time updates
   - Sync notifications
   - Connection management

6. **SDKs & Libraries**
   - Official Python client
   - Official JavaScript client
   - Community libraries

7. **Examples**
   - Common use cases
   - Sample code
   - Integration tutorials

---

## Planned API Documentation Files

- **[README.md](README.md)** - This file (overview and index)
- **[authentication.md](authentication.md)** - TODO - Auth endpoints and JWT
- **[tasks.md](tasks.md)** - TODO - Tasks CRUD operations
- **[projects.md](projects.md)** - TODO - Projects endpoints
- **[time-entries.md](time-entries.md)** - TODO - Time tracking API
- **[users.md](users.md)** - TODO - User management
- **[errors.md](errors.md)** - TODO - Error codes and handling
- **[rate-limiting.md](rate-limiting.md)** - TODO - Rate limits
- **[webhooks.md](webhooks.md)** - TODO (Phase 3) - Webhook events

---

## When to Write This

**Phase 1 (Q4 2025 - Q1 2026):**
- ✅ Authentication
- ✅ Tasks API
- ✅ Basic error handling
- ✅ OpenAPI/Swagger spec (auto-generated)

**Phase 2 (Q2 2026):**
- Projects API
- Time Entries API
- Complete error reference
- Rate limiting docs

**Phase 3 (Q3 2026):**
- WebSocket API
- Webhooks
- Advanced features

---

## Auto-Generation

**FastAPI provides automatic API documentation!**

Current auto-generated docs available at:
- **Swagger UI:** `http://localhost:8000/docs`
- **ReDoc:** `http://localhost:8000/redoc`
- **OpenAPI JSON:** `http://localhost:8000/openapi.json`

This manual documentation should:
- Supplement auto-generated docs
- Provide real-world examples
- Explain authentication flow
- Show integration patterns
- Include SDKs and libraries

---

## Format

All API docs should include:

**For each endpoint:**
```markdown
### POST /api/v1/tasks

Create a new task.

**Authentication:** Required (JWT)

**Request Body:**
```json
{
  "title": "Write documentation",
  "project_id": "uuid-here",
  "priority": "high"
}
```

**Response (201 Created):**
```json
{
  "id": "uuid-here",
  "title": "Write documentation",
  "status": "todo",
  "created_at": "2025-10-05T12:00:00Z"
}
```

**Errors:**
- 401 Unauthorized - Invalid or missing token
- 422 Unprocessable Entity - Invalid request body
```

---

## Related Documentation

- [ARCHITECTURE.md](../../ARCHITECTURE.md) - Technical architecture
- [CONTRIBUTING.md](../../CONTRIBUTING.md) - Development setup
- Auto-generated docs at `/docs` and `/redoc`

---

**Last Updated:** October 2025
**Status:** Placeholder - To be written alongside API development
