#!/bin/bash

# Test all packages and apps in the Altair monorepo
set -e

echo "🧪 Running tests for Altair monorepo..."
echo ""

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Track overall success
FAILED=0

# Test altair-ui package
echo -e "${YELLOW}Testing altair-ui...${NC}"
cd packages/altair-ui
if flutter test; then
    echo -e "${GREEN}✓ altair-ui tests passed${NC}"
else
    echo -e "${RED}✗ altair-ui tests failed${NC}"
    FAILED=1
fi
echo ""
cd ../..

# Test altair-core package
echo -e "${YELLOW}Testing altair-core...${NC}"
cd packages/altair-core
if flutter test; then
    echo -e "${GREEN}✓ altair-core tests passed${NC}"
else
    echo -e "${RED}✗ altair-core tests failed${NC}"
    FAILED=1
fi
echo ""
cd ../..

# Test altair-auth package
echo -e "${YELLOW}Testing altair-auth (requires mock generation)...${NC}"
cd packages/altair-auth
echo "Generating mocks..."
flutter pub run build_runner build --delete-conflicting-outputs > /dev/null 2>&1
if flutter test; then
    echo -e "${GREEN}✓ altair-auth tests passed${NC}"
else
    echo -e "${RED}✗ altair-auth tests failed${NC}"
    FAILED=1
fi
echo ""
cd ../..

# Test altair_guidance app
echo -e "${YELLOW}Testing altair_guidance app...${NC}"
cd apps/altair_guidance
if flutter test; then
    echo -e "${GREEN}✓ altair_guidance tests passed${NC}"
else
    echo -e "${RED}✗ altair_guidance tests failed${NC}"
    FAILED=1
fi
echo ""
cd ../..

# Final summary
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}✓ All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}✗ Some tests failed${NC}"
    exit 1
fi
