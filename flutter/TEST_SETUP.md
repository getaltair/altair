# Testing the Flutter API Client

This guide will help you test the API client setup with your FastAPI backend.

## Prerequisites

### 1. Start the FastAPI Backend

```bash
# Navigate to backend directory
cd /home/rghamilton3/workspace/getaltair/altair/backend

# Start the server with uvicorn
uv run uvicorn altair.main:app --reload --port 8000

# You should see:
# INFO:     Uvicorn running on http://127.0.0.1:8000
# INFO:     Application startup complete.
```

### 2. Verify Backend is Running

Open in browser: http://localhost:8000/docs

You should see the FastAPI interactive documentation (Swagger UI).

## Running the Tests

### Option 1: Run All Auth Tests

```bash
# From the flutter directory
cd /home/rghamilton3/workspace/getaltair/altair/flutter

# Run the integration tests
flutter test test/auth_integration_test.dart
```

### Option 2: Run with Verbose Output

```bash
flutter test test/auth_integration_test.dart --reporter expanded
```

## What the Tests Do

The integration test (`test/auth_integration_test.dart`) verifies:

1. **✅ User Registration**
   - Creates a new user with unique email
   - Verifies user data is returned correctly

2. **✅ User Login**
   - Logs in with email/password
   - Receives access + refresh tokens
   - Tokens are saved to secure storage

3. **✅ Authenticated Requests**
   - Fetches current user info using access token
   - Verifies token is automatically attached to requests

4. **✅ Logout**
   - Calls backend logout endpoint
   - Clears local token storage
   - Verifies tokens are removed

5. **✅ Error Handling**
   - Invalid credentials are rejected
   - Duplicate email registration fails

## Expected Output

```
🧪 Starting auth integration test...
📧 Test email: test_1728147890123@example.com

1️⃣ Registering new user...
✅ Registered user: test_1728147890123@example.com (ID: abc-123...)

2️⃣ Logging in...
✅ Logged in successfully
   Access token: eyJhbGciOiJIUzI1NiIs...
   Refresh token: eyJhbGciOiJIUzI1NiIs...
   Token type: bearer
   Expires in: 1800s

3️⃣ Fetching current user info...
✅ Got current user: test_1728147890123@example.com

4️⃣ Checking login status...
✅ Is logged in: true

5️⃣ Logging out...
✅ Logged out successfully
✅ Is still logged in: false

🎉 All tests passed!
```

## Troubleshooting

### Backend Connection Failed

**Error:** `DioException: Connection refused`

**Fix:** Make sure backend is running on http://localhost:8000

```bash
cd backend
uv run uvicorn altair.main:app --reload
```

### Database Not Found

**Error:** `sqlalchemy.exc.OperationalError`

**Fix:** Run database migrations

```bash
cd backend
uv run alembic upgrade head
```

### Port Already in Use

**Error:** `Address already in use`

**Fix:** Find and kill the process using port 8000

```bash
lsof -ti:8000 | xargs kill -9
```

## Next Steps After Tests Pass

Once the integration tests pass, you can proceed to:

1. **Phase 3: Auth Interceptor**
   - Automatic token refresh on 401 errors
   - Request retry with new tokens
   - Race condition handling

2. **Phase 4: Error Handling**
   - User-friendly error messages
   - Network error detection
   - Offline queue implementation

3. **Build Login UI**
   - Login/register screens
   - Auth state management with Riverpod
   - Protected routes

## Manual Testing in Terminal

You can also test manually using curl:

```bash
# Register
curl -X POST http://localhost:8000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"password123"}'

# Login
curl -X POST http://localhost:8000/api/auth/login \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "username=test@example.com&password=password123"

# Get current user (replace TOKEN with access_token from login)
curl -X GET http://localhost:8000/api/auth/me \
  -H "Authorization: Bearer TOKEN"
```
