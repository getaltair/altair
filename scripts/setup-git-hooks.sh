#!/bin/bash

# Setup script for installing git hooks
# Run this script to configure git to use the project's pre-commit hooks

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
HOOKS_DIR="$SCRIPT_DIR/git-hooks"

echo "Setting up git hooks..."

# Configure git to use the hooks directory
git config core.hooksPath "$HOOKS_DIR"

# Make hooks executable
chmod +x "$HOOKS_DIR"/*

echo "Git hooks installed successfully!"
echo "The following hooks are now active:"
ls -1 "$HOOKS_DIR"
echo ""
echo "To disable hooks, run: git config --unset core.hooksPath"
