#!/bin/bash
# Pre-commit hook to run flutter analyze on all Flutter packages
# This ensures local commits match CI pipeline Flutter analysis

set -e

# Color output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Running Flutter analyze on all packages...${NC}"

# Track if any analysis fails
FAILED=0

# List of Flutter packages/apps to analyze (matches CI pipeline)
FLUTTER_PROJECTS=(
  "packages/altair-core"
  "packages/altair-auth"
  "packages/altair-ui"
  "packages/altair-db-service"
  "apps/altair_guidance"
)

# Run flutter analyze on each project
for PROJECT in "${FLUTTER_PROJECTS[@]}"; do
  if [ -d "$PROJECT" ]; then
    echo ""
    echo -e "${YELLOW}Analyzing $PROJECT...${NC}"

    cd "$PROJECT"

    # Get dependencies first (silently)
    flutter pub get > /dev/null 2>&1

    # Run flutter analyze (same as CI)
    if flutter analyze; then
      echo -e "${GREEN}✓ $PROJECT passed${NC}"
    else
      echo -e "${RED}✗ $PROJECT failed${NC}"
      FAILED=1
    fi

    cd - > /dev/null
  fi
done

echo ""

if [ $FAILED -eq 1 ]; then
  echo -e "${RED}Flutter analyze failed. Please fix the issues above.${NC}"
  exit 1
else
  echo -e "${GREEN}All Flutter packages passed analysis!${NC}"
  exit 0
fi
