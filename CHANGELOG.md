# Changelog

All notable changes to the Altair project will be documented in this file.

## [Current] - 2025-10-04

### feat(landing): build production-ready landing page with newsletter signup

- Transform index.html from placeholder to full-featured landing page
- Implement brand-compliant design using Inter font and official color palette (#3B82F6, #0F172A)
- Add hero section with animated diamond star icon and project tagline
- Add Buttondown newsletter integration for email signups at https://buttondown.com/getaltair
- Add 6 feature cards showcasing ADHD-friendly capabilities (quick capture, focus sessions, task decomposition, cognitive load tracking, offline-first, celebration)
- Add "Why Altair?" mission statement section
- Add "Built in Public" community section with GitHub CTAs
- Implement fully responsive design with mobile-first approach
- Add accessibility features: reduced motion support, WCAG AA contrast ratios, semantic HTML
- Add SEO meta tags and Open Graph/Twitter social sharing metadata
- Add favicon integration with multiple formats (SVG, PNG 512/256/64)
- Add footer with documentation links and licensing information
- Style landing page following ADHD-friendly principles: 1.6 line height, clear hierarchy, high contrast

### docs: restructure documentation with comprehensive root-level guides

- Add ARCHITECTURE.md with detailed system design, tech stack, and deployment strategies
- Add CONTRIBUTING.md with comprehensive contribution guidelines and ADHD-friendly tips
- Add DOCUMENTATION_INDEX.md as central navigation hub for all documentation
- Add FEATURES.md with in-depth ADHD-specific feature explanations and design principles
- Add ROADMAP.md with development phases, milestones, and future vision
- Add SECURITY.md with security policy, vulnerability reporting, and best practices
- Move JWT_AUTH_IMPLEMENTATION_PLAN.md from docs/ to root level for better visibility
- Streamline README.md to focus on quick start and project overview (reduced from 384+ lines)
- Remove outdated docs: ALTAIR_DOGFOODING.md, ALTAIR_QUICKSTART.md, FLUTTER_FRONTEND.md, FULLSTACK_INTEGRATION.md
- Remove backend/README.md (consolidated into main documentation)
- Add index.html landing page placeholder

### feat(auth): implement logout endpoint with token blacklisting and rate limiting

- Add logout endpoint that revokes access tokens via Redis blacklist
- Implement token blacklist checking in get_current_user dependency
- Add Redis client module for token revocation and caching
- Add rate limiting to auth endpoints (register: 3/min, login: 5/min, refresh: 10/min, logout: 10/min)
- Add slowapi dependency for rate limiting functionality
- Update .env.example with REFRESH_TOKEN_EXPIRE_DAYS and improved documentation
- Add user ownership to all task operations (create, read, update, list)
- Ensure tasks are filtered by user_id to enforce data isolation

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
