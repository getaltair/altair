#!/usr/bin/env bash
# zitadel-setup.sh — Idempotent Zitadel OIDC initial configuration
#
# Usage:
#   ZITADEL_ADMIN_PAT=<pat> ./infra/scripts/zitadel-setup.sh
#
# Environment variables:
#   ZITADEL_BASE_URL   Base URL for Zitadel (default: http://localhost:8080)
#   ZITADEL_ADMIN_PAT  Personal Access Token for the initial admin user (required)

set -euo pipefail

# ---------------------------------------------------------------------------
# Configuration
# ---------------------------------------------------------------------------
ZITADEL_BASE_URL="${ZITADEL_BASE_URL:-http://localhost:8080}"
ORG_NAME="Altair"
PROJECT_NAME="altair-server"
APP_NAME="altair-web"
MACHINE_USER_NAME="powersync-validator"
REDIRECT_URI="http://localhost:5173/auth/callback"
POST_LOGOUT_URI="http://localhost:5173"

# ---------------------------------------------------------------------------
# Validate prerequisites
# ---------------------------------------------------------------------------
if [[ -z "${ZITADEL_ADMIN_PAT:-}" ]]; then
  echo "ERROR: ZITADEL_ADMIN_PAT environment variable is required." >&2
  echo "       Set it to the Personal Access Token of the initial Zitadel admin." >&2
  exit 1
fi

if ! command -v curl &>/dev/null; then
  echo "ERROR: curl is required but not found in PATH." >&2
  exit 1
fi

if ! command -v jq &>/dev/null; then
  echo "ERROR: jq is required but not found in PATH." >&2
  exit 1
fi

AUTH_HEADER="Authorization: Bearer ${ZITADEL_ADMIN_PAT}"

# ---------------------------------------------------------------------------
# Helper: make an authenticated API call and return the response body.
# Usage: api_call <METHOD> <PATH> [body]
# ---------------------------------------------------------------------------
api_call() {
  local method="$1"
  local path="$2"
  local body="${3:-}"
  local url="${ZITADEL_BASE_URL}${path}"

  if [[ -n "$body" ]]; then
    curl --silent --show-error --fail-with-body \
      -X "$method" "$url" \
      -H "$AUTH_HEADER" \
      -H "Content-Type: application/json" \
      -d "$body"
  else
    curl --silent --show-error --fail-with-body \
      -X "$method" "$url" \
      -H "$AUTH_HEADER" \
      -H "Content-Type: application/json"
  fi
}

# ---------------------------------------------------------------------------
# Step 1: Wait for Zitadel to be ready (up to 60 seconds)
# ---------------------------------------------------------------------------
echo "==> Waiting for Zitadel to be ready at ${ZITADEL_BASE_URL} ..."

READY_URL="${ZITADEL_BASE_URL}/debug/healthz/ready"
WAIT_SECONDS=60
ELAPSED=0
INTERVAL=3

until curl --silent --fail "$READY_URL" &>/dev/null; do
  if [[ $ELAPSED -ge $WAIT_SECONDS ]]; then
    echo "ERROR: Zitadel did not become ready within ${WAIT_SECONDS}s." >&2
    exit 1
  fi
  echo "    ... not ready yet (${ELAPSED}s elapsed). Retrying in ${INTERVAL}s."
  sleep $INTERVAL
  ELAPSED=$((ELAPSED + INTERVAL))
done

echo "    Zitadel is ready."

# ---------------------------------------------------------------------------
# Step 2: Resolve default organisation ID (needed for project/app scoping)
# ---------------------------------------------------------------------------
# Zitadel's v2 API requires an organisation context for project and app
# creation. We query for the org by name and fall back to creating it.
echo "==> Checking for organisation '${ORG_NAME}' ..."

ORG_SEARCH_RESPONSE=$(api_call GET \
  "/v2/organisations?filters%5B0%5D%5Bquery%5D%5BnameQuery%5D%5Bname%5D=$(python3 -c "import urllib.parse; print(urllib.parse.quote('${ORG_NAME}')" 2>/dev/null || printf '%s' "${ORG_NAME}")" \
  || true)

# Simpler approach: use POST search endpoint which is more reliable
ORG_SEARCH_RESPONSE=$(curl --silent --show-error \
  -X POST "${ZITADEL_BASE_URL}/v2/organisations/_search" \
  -H "$AUTH_HEADER" \
  -H "Content-Type: application/json" \
  -d "{
    \"queries\": [{
      \"nameQuery\": {
        \"name\": \"${ORG_NAME}\",
        \"method\": \"TEXT_QUERY_METHOD_EQUALS\"
      }
    }]
  }" || true)

ORG_ID=$(echo "$ORG_SEARCH_RESPONSE" | jq -r '.result[0].id // empty' 2>/dev/null || true)

if [[ -n "$ORG_ID" ]]; then
  echo "    Organisation '${ORG_NAME}' already exists (id=${ORG_ID})."
else
  # ---------------------------------------------------------------------------
  # Step 3: Create organisation "Altair"
  # ---------------------------------------------------------------------------
  echo "==> Creating organisation '${ORG_NAME}' ..."
  ORG_RESPONSE=$(api_call POST "/v2/organisations" \
    "{\"name\": \"${ORG_NAME}\"}")
  ORG_ID=$(echo "$ORG_RESPONSE" | jq -r '.organisationId')
  echo "    Created organisation (id=${ORG_ID})."
fi

# ---------------------------------------------------------------------------
# Step 4: Create project "altair-server" (idempotent)
# ---------------------------------------------------------------------------
echo "==> Checking for project '${PROJECT_NAME}' ..."

PROJECT_SEARCH_RESPONSE=$(curl --silent --show-error \
  -X POST "${ZITADEL_BASE_URL}/v2/projects/_search" \
  -H "$AUTH_HEADER" \
  -H "Content-Type: application/json" \
  -H "x-zitadel-orgid: ${ORG_ID}" \
  -d "{
    \"queries\": [{
      \"nameQuery\": {
        \"name\": \"${PROJECT_NAME}\",
        \"method\": \"TEXT_QUERY_METHOD_EQUALS\"
      }
    }]
  }" || true)

PROJECT_ID=$(echo "$PROJECT_SEARCH_RESPONSE" | jq -r '.result[0].id // empty' 2>/dev/null || true)

if [[ -n "$PROJECT_ID" ]]; then
  echo "    Project '${PROJECT_NAME}' already exists (id=${PROJECT_ID})."
else
  echo "==> Creating project '${PROJECT_NAME}' ..."
  PROJECT_RESPONSE=$(curl --silent --show-error --fail-with-body \
    -X POST "${ZITADEL_BASE_URL}/v2/projects" \
    -H "$AUTH_HEADER" \
    -H "Content-Type: application/json" \
    -H "x-zitadel-orgid: ${ORG_ID}" \
    -d "{\"name\": \"${PROJECT_NAME}\"}")
  PROJECT_ID=$(echo "$PROJECT_RESPONSE" | jq -r '.id')
  echo "    Created project (id=${PROJECT_ID})."
fi

# ---------------------------------------------------------------------------
# Step 5: Create OIDC web application with PKCE (idempotent)
# ---------------------------------------------------------------------------
echo "==> Checking for OIDC application '${APP_NAME}' ..."

APP_SEARCH_RESPONSE=$(curl --silent --show-error \
  -X POST "${ZITADEL_BASE_URL}/v2/projects/${PROJECT_ID}/apps/_search" \
  -H "$AUTH_HEADER" \
  -H "Content-Type: application/json" \
  -H "x-zitadel-orgid: ${ORG_ID}" \
  -d "{
    \"queries\": [{
      \"nameQuery\": {
        \"name\": \"${APP_NAME}\",
        \"method\": \"TEXT_QUERY_METHOD_EQUALS\"
      }
    }]
  }" || true)

CLIENT_ID=$(echo "$APP_SEARCH_RESPONSE" | jq -r '.result[0].oidcConfig.clientId // empty' 2>/dev/null || true)
APP_ID=$(echo "$APP_SEARCH_RESPONSE" | jq -r '.result[0].id // empty' 2>/dev/null || true)

if [[ -n "$CLIENT_ID" ]]; then
  echo "    OIDC application '${APP_NAME}' already exists (clientId=${CLIENT_ID})."
else
  echo "==> Creating OIDC application '${APP_NAME}' with PKCE ..."
  APP_RESPONSE=$(curl --silent --show-error --fail-with-body \
    -X POST "${ZITADEL_BASE_URL}/v2/projects/${PROJECT_ID}/apps/oidc" \
    -H "$AUTH_HEADER" \
    -H "Content-Type: application/json" \
    -H "x-zitadel-orgid: ${ORG_ID}" \
    -d "{
      \"name\": \"${APP_NAME}\",
      \"redirectUris\": [\"${REDIRECT_URI}\"],
      \"postLogoutRedirectUris\": [\"${POST_LOGOUT_URI}\"],
      \"responseTypes\": [\"OIDC_RESPONSE_TYPE_CODE\"],
      \"grantTypes\": [\"OIDC_GRANT_TYPE_AUTHORIZATION_CODE\"],
      \"appType\": \"OIDC_APP_TYPE_WEB\",
      \"authMethodType\": \"OIDC_AUTH_METHOD_TYPE_NONE\",
      \"devMode\": true
    }")
  CLIENT_ID=$(echo "$APP_RESPONSE" | jq -r '.clientId')
  APP_ID=$(echo "$APP_RESPONSE" | jq -r '.appId')
  echo "    Created OIDC application (appId=${APP_ID}, clientId=${CLIENT_ID})."
fi

# ---------------------------------------------------------------------------
# Step 6: Create machine user for PowerSync JWT validation (idempotent)
# ---------------------------------------------------------------------------
echo "==> Checking for machine user '${MACHINE_USER_NAME}' ..."

MACHINE_SEARCH_RESPONSE=$(curl --silent --show-error \
  -X POST "${ZITADEL_BASE_URL}/v2/users/_search" \
  -H "$AUTH_HEADER" \
  -H "Content-Type: application/json" \
  -H "x-zitadel-orgid: ${ORG_ID}" \
  -d "{
    \"queries\": [{
      \"userNameQuery\": {
        \"userName\": \"${MACHINE_USER_NAME}\",
        \"method\": \"TEXT_QUERY_METHOD_EQUALS\"
      }
    }]
  }" || true)

MACHINE_USER_ID=$(echo "$MACHINE_SEARCH_RESPONSE" | jq -r '.result[0].userId // empty' 2>/dev/null || true)

if [[ -n "$MACHINE_USER_ID" ]]; then
  echo "    Machine user '${MACHINE_USER_NAME}' already exists (id=${MACHINE_USER_ID})."
else
  echo "==> Creating machine user '${MACHINE_USER_NAME}' ..."
  MACHINE_RESPONSE=$(curl --silent --show-error --fail-with-body \
    -X POST "${ZITADEL_BASE_URL}/v2/users/machine" \
    -H "$AUTH_HEADER" \
    -H "Content-Type: application/json" \
    -H "x-zitadel-orgid: ${ORG_ID}" \
    -d "{
      \"userName\": \"${MACHINE_USER_NAME}\",
      \"name\": \"PowerSync JWT Validator\",
      \"description\": \"Service user for PowerSync JWT validation via JWKS\"
    }")
  MACHINE_USER_ID=$(echo "$MACHINE_RESPONSE" | jq -r '.userId')
  echo "    Created machine user (id=${MACHINE_USER_ID})."
fi

# ---------------------------------------------------------------------------
# Done — print .env values
# ---------------------------------------------------------------------------
JWKS_URL="${ZITADEL_BASE_URL}/oauth/v2/keys"

echo ""
echo "=== Zitadel Setup Complete ==="
echo "Add to your .env:"
echo ""
echo "ZITADEL_CLIENT_ID=${CLIENT_ID}"
echo "ZITADEL_JWKS_URL=${JWKS_URL}"
echo ""
echo "Organisation id : ${ORG_ID}"
echo "Project id      : ${PROJECT_ID}"
echo "App id          : ${APP_ID:-<existing>}"
echo "Machine user id : ${MACHINE_USER_ID}"
