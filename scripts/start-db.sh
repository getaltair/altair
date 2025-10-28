#!/bin/bash

# Start SurrealDB with Altair defaults
set -e

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Default values (matching altair-db-service config)
DEFAULT_USER="altair"
DEFAULT_PASS="altair-local-dev"
DEFAULT_DB_PATH="./altair.db"
DEFAULT_PORT="8000"
DEFAULT_BIND="0.0.0.0"

# Parse arguments
USER="${1:-$DEFAULT_USER}"
PASS="${2:-$DEFAULT_PASS}"
DB_PATH="${3:-$DEFAULT_DB_PATH}"

# Function to check if SurrealDB is installed
check_surrealdb() {
  if ! command -v surreal &> /dev/null; then
    echo -e "${RED}❌ SurrealDB is not installed${NC}"
    echo ""
    echo -e "${BLUE}Install SurrealDB:${NC}"
    echo "  curl -sSf https://install.surrealdb.com | sh"
    echo ""
    exit 1
  fi
}

# Function to check if SurrealDB is already running
check_running() {
  if curl -s "http://${DEFAULT_BIND}:${DEFAULT_PORT}/health" &> /dev/null; then
    echo -e "${YELLOW}⚠️  SurrealDB is already running on port ${DEFAULT_PORT}${NC}"
    echo ""
    echo -e "${BLUE}To stop the existing instance:${NC}"
    echo "  pkill surreal"
    echo ""
    exit 1
  fi
}

# Display banner
echo "🗄️  Starting SurrealDB for Altair"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Check if SurrealDB is installed
check_surrealdb

# Check if already running
check_running

# Display configuration
echo -e "${BLUE}Configuration:${NC}"
echo "  User:      $USER"
echo "  Password:  $(echo "$PASS" | sed 's/./*/g')"
echo "  Database:  $DB_PATH"
echo "  Bind:      ${DEFAULT_BIND}:${DEFAULT_PORT}"
echo "  Namespace: altair"
echo "  Database:  local"
echo ""

# Start SurrealDB
echo -e "${YELLOW}Starting SurrealDB...${NC}"
surreal start \
  --bind "${DEFAULT_BIND}:${DEFAULT_PORT}" \
  --user "$USER" \
  --pass "$PASS" \
  "file://${DB_PATH}" &

# Wait for service to be ready
echo -n "Waiting for service to start"
for i in {1..30}; do
  if curl -s "http://${DEFAULT_BIND}:${DEFAULT_PORT}/health" &> /dev/null; then
    echo ""
    echo -e "${GREEN}✓ SurrealDB is running${NC}"
    echo ""
    echo -e "${BLUE}Connection details:${NC}"
    echo "  WebSocket: ws://${DEFAULT_BIND}:${DEFAULT_PORT}/rpc"
    echo "  HTTP:      http://${DEFAULT_BIND}:${DEFAULT_PORT}"
    echo "  Health:    http://${DEFAULT_BIND}:${DEFAULT_PORT}/health"
    echo ""
    echo -e "${GREEN}Ready for integration tests!${NC}"
    echo ""
    echo -e "${BLUE}To stop:${NC}"
    echo "  pkill surreal"
    echo ""
    exit 0
  fi
  echo -n "."
  sleep 1
done

echo ""
echo -e "${RED}❌ Failed to start SurrealDB${NC}"
echo ""
echo -e "${BLUE}Check logs:${NC}"
echo "  ps aux | grep surreal"
echo ""
exit 1
