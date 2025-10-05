# JWT Authentication Implementation Plan

**Date:** 2025-10-04
**Status:** Planning
**Target:** Single-user MVP with JWT foundation for future multi-user support

## Overview

Implement JWT-based authentication to secure the Altair API endpoints while maintaining public access to health checks and building-in-public stats endpoints.

## Goals

1. Secure task management endpoints behind authentication
2. Enable user registration and login
3. Set foundation for future multi-user support
4. Maintain public access for monitoring and public stats

## Technical Decisions

### Authentication Strategy
- **Token Type:** JWT (JSON Web Tokens)
- **Token Storage:** Client-side (localStorage for web, secure storage for mobile)
- **Token Expiration:** 7 days (adjustable via config)
- **Refresh Strategy:** Manual re-login for MVP (can add refresh tokens later)
- **Password Hashing:** bcrypt via passlib

### User Model
```python
User:
  - id: UUID (primary key)
  - email: String (unique, indexed)
  - hashed_password: String
  - is_active: Boolean (default: True)
  - created_at: DateTime
  - updated_at: DateTime
```

### Security Configuration
- **Algorithm:** HS256
- **Secret Key:** From environment variable (SECRET_KEY)
- **Password Requirements:** Minimum 8 characters (can enhance later)
- **Email Validation:** Basic format check via Pydantic

## Implementation Steps

### Phase 1: Database & Models (30 min)

#### Step 1.1: Create User Model
**File:** `backend/altair/models/user.py`
- [ ] Create User SQLAlchemy model
- [ ] Add email column with unique constraint
- [ ] Add hashed_password column
- [ ] Add is_active boolean field
- [ ] Inherit from Base for timestamps

#### Step 1.2: Update Models Index
**File:** `backend/altair/models/__init__.py`
- [ ] Export User model for Alembic discovery

#### Step 1.3: Create Alembic Migration
```bash
cd backend
uv run alembic revision --autogenerate -m "Add users table"
uv run alembic upgrade head
```
- [ ] Review generated migration
- [ ] Verify indexes on email column
- [ ] Run migration locally
- [ ] Test rollback and re-apply

### Phase 2: Pydantic Schemas (15 min)

#### Step 2.1: Create Auth Schemas
**File:** `backend/altair/schemas/auth.py`
- [ ] UserBase (email)
- [ ] UserCreate (email, password) with validation
- [ ] UserResponse (id, email, created_at) - no password
- [ ] Token (access_token, token_type)
- [ ] TokenData (email or None)

#### Step 2.2: Update Schemas Index
**File:** `backend/altair/schemas/__init__.py`
- [ ] Export auth schemas

### Phase 3: Authentication Utilities (20 min)

#### Step 3.1: Create Auth Utilities
**File:** `backend/altair/services/auth.py`
- [ ] `verify_password(plain_password, hashed_password)` - compare passwords
- [ ] `get_password_hash(password)` - hash new passwords
- [ ] `create_access_token(data: dict, expires_delta)` - generate JWT
- [ ] `verify_token(token)` - decode and validate JWT
- [ ] Use passlib with bcrypt for hashing
- [ ] Use python-jose for JWT operations

### Phase 4: Authentication Endpoints (30 min)

#### Step 4.1: Create Auth Router
**File:** `backend/altair/api/auth.py`

**Endpoints:**
- [ ] `POST /api/auth/register` - Create new user
  - Validate email format
  - Check if user already exists
  - Hash password
  - Create user in database
  - Return user info (no token yet)

- [ ] `POST /api/auth/login` - Authenticate user
  - OAuth2PasswordRequestForm for compatibility
  - Verify email exists
  - Verify password matches
  - Generate JWT token
  - Return token response

- [ ] `GET /api/auth/me` - Get current user info (protected)
  - Requires valid JWT
  - Returns current user details

#### Step 4.2: Register Auth Router
**File:** `backend/altair/main.py`
- [ ] Import auth router
- [ ] Add to app with `/api/auth` prefix

### Phase 5: Authentication Dependency (15 min)

#### Step 5.1: Create Auth Dependency
**File:** `backend/altair/dependencies.py`
- [ ] `get_current_user(token: str)` - extract and verify JWT
- [ ] Decode token
- [ ] Extract user email from token
- [ ] Query user from database
- [ ] Raise 401 if invalid
- [ ] Return User object

#### Step 5.2: Optional Active User Dependency
**File:** `backend/altair/dependencies.py`
- [ ] `get_current_active_user` - checks is_active flag
- [ ] Wraps get_current_user
- [ ] Raises 403 if user not active

### Phase 6: Protect Existing Endpoints (20 min)

#### Step 6.1: Update Task Endpoints
**File:** `backend/altair/api/tasks.py`
- [ ] Add `current_user: User = Depends(get_current_active_user)` to all endpoints
- [ ] Update task creation to include user_id
- [ ] Filter task queries by user_id
- [ ] Update quick-capture endpoint
- [ ] Update list tasks endpoint
- [ ] Update create task endpoint
- [ ] Update get task endpoint
- [ ] Update update task endpoint

#### Step 6.2: Add user_id to Task Model
**File:** `backend/altair/models/task.py`
- [ ] Add user_id column with ForeignKey to users table
- [ ] Add relationship to User model
- [ ] Create migration for schema change

#### Step 6.3: Update Task Schemas
**File:** `backend/altair/schemas/task.py`
- [ ] Add user_id to TaskResponse (optional for now)
- [ ] Consider whether to expose user_id in API

### Phase 7: Testing & Validation (30 min)

#### Step 7.1: Manual Testing Flow
- [ ] Test user registration with valid email
- [ ] Test registration with duplicate email (should fail)
- [ ] Test login with correct credentials
- [ ] Test login with wrong password (should fail)
- [ ] Test accessing protected endpoint without token (should fail)
- [ ] Test accessing protected endpoint with valid token (should work)
- [ ] Test accessing protected endpoint with expired token (should fail)
- [ ] Test quick-capture with authentication
- [ ] Test listing tasks shows only current user's tasks

#### Step 7.2: API Documentation
- [ ] Verify Swagger UI shows auth endpoints
- [ ] Test "Authorize" button in Swagger UI
- [ ] Verify protected endpoints show lock icon

#### Step 7.3: Database Verification
- [ ] Check users table created correctly
- [ ] Verify password is hashed (not plaintext)
- [ ] Verify user_id foreign key on tasks table
- [ ] Check indexes exist on email column

### Phase 8: Configuration & Security (15 min)

#### Step 8.1: Verify Environment Variables
**Railway Configuration:**
- [ ] SECRET_KEY is set (use `openssl rand -hex 32`)
- [ ] DATABASE_URL is set
- [ ] REDIS_URL is set (for future use)

#### Step 8.2: Update Config Validation
**File:** `backend/altair/config.py`
- [ ] Ensure SECRET_KEY validator works
- [ ] Set DEBUG=False in Railway
- [ ] Add ACCESS_TOKEN_EXPIRE_DAYS setting

### Phase 9: Documentation (15 min)

#### Step 9.1: Update API Documentation
- [ ] Add authentication section to README
- [ ] Document registration flow
- [ ] Document login flow
- [ ] Document how to use bearer tokens

#### Step 9.2: Update CHANGELOG
- [ ] Add entry for JWT authentication feature

## File Checklist

### New Files to Create
- [ ] `backend/altair/models/user.py`
- [ ] `backend/altair/schemas/auth.py`
- [ ] `backend/altair/services/auth.py`
- [ ] `backend/altair/api/auth.py`
- [ ] `backend/altair/dependencies.py` (if not exists)

### Files to Modify
- [ ] `backend/altair/models/__init__.py`
- [ ] `backend/altair/models/task.py`
- [ ] `backend/altair/schemas/__init__.py`
- [ ] `backend/altair/schemas/task.py`
- [ ] `backend/altair/api/tasks.py`
- [ ] `backend/altair/main.py`
- [ ] `backend/altair/config.py`

### Migrations to Create
- [ ] Add users table
- [ ] Add user_id to tasks table

## Testing Checklist

### Registration
```bash
curl -X POST "http://localhost:8000/api/auth/register" \
  -H "Content-Type: application/json" \
  -d '{"email": "you@example.com", "password": "testpassword123"}'
```

### Login
```bash
curl -X POST "http://localhost:8000/api/auth/login" \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "username=you@example.com&password=testpassword123"
```

### Access Protected Endpoint
```bash
TOKEN="your-token-here"
curl -X GET "http://localhost:8000/api/tasks" \
  -H "Authorization: Bearer $TOKEN"
```

### Quick Capture with Auth
```bash
curl -X POST "http://localhost:8000/api/tasks/quick-capture?text=Test%20task" \
  -H "Authorization: Bearer $TOKEN"
```

## Public Endpoints (No Auth Required)

These endpoints should remain publicly accessible:
- `GET /` - API info
- `GET /health` - Health check
- `GET /api/public/stats/{username}` - Public dogfooding stats (future)

## Security Considerations

### Immediate
- [x] Use environment variable for SECRET_KEY (already configured)
- [ ] Hash passwords with bcrypt
- [ ] Validate email format
- [ ] Use HTTPS in production (Railway handles this)

### Future Enhancements
- [ ] Add refresh tokens for better UX
- [ ] Implement rate limiting on auth endpoints
- [ ] Add password strength requirements
- [ ] Add email verification
- [ ] Add password reset flow
- [ ] Add account deletion
- [ ] Add session management
- [ ] Add OAuth2 providers (Google, GitHub)

## Rollout Plan

### Development
1. Implement all features locally
2. Test with local PostgreSQL
3. Verify migrations work correctly

### Staging (Railway)
1. Deploy to Railway
2. Run migrations on Railway database
3. Create test user account
4. Verify all endpoints work

### Production
1. Same as staging (Railway is production for MVP)
2. Create your personal user account
3. Update Flutter app to use auth tokens
4. Monitor error logs

## Time Estimate

- **Phase 1:** 30 minutes (Database & Models)
- **Phase 2:** 15 minutes (Schemas)
- **Phase 3:** 20 minutes (Auth Utilities)
- **Phase 4:** 30 minutes (Auth Endpoints)
- **Phase 5:** 15 minutes (Dependencies)
- **Phase 6:** 20 minutes (Protect Endpoints)
- **Phase 7:** 30 minutes (Testing)
- **Phase 8:** 15 minutes (Security Config)
- **Phase 9:** 15 minutes (Documentation)

**Total:** ~3 hours (including testing and verification)

## Success Criteria

- [ ] Can register a new user
- [ ] Can login and receive JWT token
- [ ] Cannot access task endpoints without valid token
- [ ] Can access task endpoints with valid token
- [ ] Tasks are scoped to the authenticated user
- [ ] Public endpoints remain accessible
- [ ] All migrations run successfully
- [ ] Password is never stored in plaintext
- [ ] Token expires after configured duration
- [ ] Swagger UI shows auth properly

## Next Steps After Auth

Once JWT authentication is complete:
1. Implement task state transitions
2. Build public stats endpoint
3. Add focus timer with WebSocket
4. Create Flutter authentication flow
5. Implement public profile page for dogfooding transparency

---

**Remember:** Start simple, test thoroughly, ship incrementally. This is the foundation for all future features.
