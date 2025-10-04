# Changelog

All notable changes to the Altair project will be documented in this file.

## [Current] - 2025-10-04

### refactor(brand): rebrand from Polaris to Altair

- Rename all Python packages from `polaris` to `altair` throughout backend
- Update project name, tagline ("Where focus takes flight"), and branding across all docs
- Add comprehensive brand assets: diamond star logo/icon, social media banners, favicon generator
- Add brand guidelines (434-line comprehensive manual with colors, typography, voice, accessibility)
- Add brand assets guide with platform-specific usage instructions
- Add landing page template with email signup and ADHD-friendly design
- Update README with Altair branding, new tagline, and getaltair.app links
- Rename POLARIS_PYTHON_CONTEXT.md to ALTAIR_PYTHON_CONTEXT.md
- Rename docs: POLARIS_DOGFOODING.md → ALTAIR_DOGFOODING.md, POLARIS_QUICKSTART.md → ALTAIR_QUICKSTART.md
- Update Flutter app configuration: package name com.rghsoftware.polaris → com.getaltair.altair
- Add JWT authentication implementation plan documentation
- Update all import statements, configuration, and Docker files with new naming
- Update CLAUDE.md development guidelines with Altair branding

### build(railway): migrate from Nixpacks to Dockerfile for deployment

- Add multi-stage Dockerfile with UV package manager support
- Add .dockerignore to optimize Docker build context
- Update railway.json to use DOCKERFILE builder instead of NIXPACKS
- Fix start command to use correct altair.main:app module path
- Optimize for production with non-root user and minimal runtime image

### build(railway): add Railway deployment configuration and dependencies

- Add railway.json with Nixpacks builder configuration
- Add .env.example with database, Redis, and secret key templates
- Move requirements.txt to root for Railway deployment compatibility
- Configure uvicorn start command for Railway with dynamic port binding
- Set restart policy with failure handling and max retries

## [2025-10-03]

### docs: add GNU AGPL v3 license and comprehensive project README

- Add LICENSE.md with full GNU Affero General Public License v3 text
- Add README.md with project overview, features, tech stack, and quickstart guide
- Establish AGPL licensing for network copyleft protection

### chore(init): initial project setup for Python/FastAPI ADHD-friendly task manager

- Add project structure with backend Python package layout
- Add FastAPI dependencies and development tools in pyproject.toml
- Add comprehensive CLAUDE.md with development guidelines and patterns
- Add ALTAIR_PYTHON_CONTEXT.md with architecture decisions and MVP roadmap
- Add extensive documentation for Flutter frontend, fullstack integration, dogfooding strategy, and quickstart guide
- Add custom Claude Code /commit command for conventional commits
- Add .gitignore for Python, UV, Docker, Flutter, and development artifacts
- Add Python 3.12 version specification
- Add mise.toml for development environment configuration
- Add placeholder backend entry point (main.py) and package structure
