#!/bin/bash
# Pre-commit hook to run dart format check on all Dart files
# This ensures local commits match CI pipeline formatting requirements
# Uses --set-exit-if-changed to fail if any files need formatting (same as CI)

set -e

# Color output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Checking Dart formatting (matching CI)...${NC}"

# Run dart format with --set-exit-if-changed (same as CI)
# This will exit with code 1 if any files need formatting
if dart format --set-exit-if-changed .; then
  echo -e "${GREEN}✓ All Dart files are properly formatted!${NC}"
  exit 0
else
  echo ""
  echo -e "${RED}✗ Dart formatting check failed!${NC}"
  echo -e "${YELLOW}Some files need formatting. Run:${NC}"
  echo -e "  ${GREEN}dart format .${NC}"
  echo ""
  exit 1
fi
