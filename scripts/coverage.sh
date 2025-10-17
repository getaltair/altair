#!/bin/bash

# Generate coverage reports for all packages
set -e

echo "📊 Generating coverage reports for Altair monorepo..."
echo ""

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Create coverage directory
mkdir -p coverage

# Test altair-ui with coverage
echo -e "${YELLOW}Generating coverage for altair-ui...${NC}"
cd packages/altair-ui
flutter test --coverage
mv coverage/lcov.info ../../coverage/altair-ui.lcov.info
echo -e "${GREEN}✓ altair-ui coverage generated${NC}"
echo ""
cd ../..

# Test altair-core with coverage
echo -e "${YELLOW}Generating coverage for altair-core...${NC}"
cd packages/altair-core
flutter test --coverage
mv coverage/lcov.info ../../coverage/altair-core.lcov.info
echo -e "${GREEN}✓ altair-core coverage generated${NC}"
echo ""
cd ../..

# Test altair-auth with coverage
echo -e "${YELLOW}Generating coverage for altair-auth...${NC}"
cd packages/altair-auth
echo "Generating mocks..."
flutter pub run build_runner build --delete-conflicting-outputs > /dev/null 2>&1
flutter test --coverage
mv coverage/lcov.info ../../coverage/altair-auth.lcov.info
echo -e "${GREEN}✓ altair-auth coverage generated${NC}"
echo ""
cd ../..

# Test altair_guidance with coverage
echo -e "${YELLOW}Generating coverage for altair_guidance...${NC}"
cd apps/altair_guidance
flutter test --coverage
mv coverage/lcov.info ../../coverage/altair-guidance.lcov.info
echo -e "${GREEN}✓ altair_guidance coverage generated${NC}"
echo ""
cd ../..

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${GREEN}✓ Coverage reports generated in coverage/ directory${NC}"
echo ""
echo -e "${BLUE}To view HTML coverage reports:${NC}"
echo "  1. Install lcov: brew install lcov (macOS) or apt-get install lcov (Linux)"
echo "  2. Generate HTML: genhtml coverage/altair-ui.lcov.info -o coverage/html/altair-ui"
echo "  3. Open in browser: open coverage/html/altair-ui/index.html"
echo ""
echo -e "${BLUE}Coverage files:${NC}"
ls -lh coverage/*.lcov.info
