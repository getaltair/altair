# Changelog

All notable changes to the Altair project will be documented in this file.

## [Current] - 2025-10-05

### ci(github): optimize workflow with path-based job filtering

- Add path filtering using dorny/paths-filter to detect backend changes
- Implement conditional job execution - lint and test only run when backend files change
- Add gatekeeper job (all-checks-passed) that validates results or confirms appropriate skips
- Improve CI efficiency by skipping unnecessary jobs when only docs/frontend change
- Add comprehensive status reporting in final validation step
- Ensure CI still acts as proper merge gate with always() condition on gatekeeper

### ci(github): add issue templates and pull request template

- Add ADHD-specific feedback issue template for user experience insights
- Add bug report template with ADHD-friendly note
- Add feature request template with ADHD context section
- Add pull request template with comprehensive checklist
- Update CLAUDE.md to always sign commits

## [2025-10-04]

### fix(diagrams): correct Mermaid syntax errors for GitHub rendering

- Fix quadrant chart labels in 04-roadmap-planning.md to remove special characters
  - Change "Quick Wins (DO FIRST!)" to "Quick Wins - Do First"
  - Change "Avoid/Defer" to "Avoid or Defer"
- Fix flowchart node label in 03-user-flows.md to remove nested quotes
  - Change 'Show "Working Offline" Badge' to "Show Working Offline Badge"
- Ensures all Mermaid diagrams render correctly on GitHub

### docs: create comprehensive documentation system with diagrams and guides

- **Core Documentation Updates:**
  - Expand ARCHITECTURE.md with detailed system design, technology stack, and deployment architecture
  - Enhance CONTRIBUTING.md with development workflows, code standards, and ADHD-friendly contribution tips
  - Restructure FEATURES.md with in-depth explanations of ADHD-specific features and design principles
  - Overhaul README.md for improved project overview and quick start instructions
  - Expand ROADMAP.md with detailed development phases, milestones, and future vision
  - Update DOCUMENTATION_INDEX.md to serve as central navigation hub

- **New Documentation Files:**
  - Add FAQ.md with common questions and answers about Altair
  - Add QUICK_START.md with streamlined setup instructions for new users
  - Add UPDATE_SUMMARY.md documenting recent major changes
  - Add CODE_OF_CONDUCT.md for community standards and expectations

- **Visual Documentation (diagrams/):**
  - Add 01-system-architecture.md with high-level architecture diagrams
  - Add 02-database-schema-erd.md with complete database entity-relationship diagrams
  - Add 03-user-flows.md with ADHD-friendly user journey visualizations
  - Add 04-roadmap-planning.md with visual development timeline
  - Add 05-component-architecture.md with detailed component hierarchy diagrams
  - Add 06-deployment-operations.md with deployment architecture and infrastructure diagrams
  - Add diagrams/README.md as navigation hub for visual documentation
  - Add diagrams/SUMMARY.md with comprehensive diagram overview
  - Add diagrams/index.html for browsing diagrams in web browser

- **API Documentation (docs/api/):**
  - Add docs/api/README.md with API overview and conventions
  - Add docs/api/authentication.md with auth endpoints and flows
  - Add docs/api/projects.md with project management endpoints
  - Add docs/api/tasks.md with task management and quick capture endpoints

- **User Guide (docs/user-guide/):**
  - Add docs/user-guide/README.md as user documentation hub
  - Add docs/user-guide/getting-started.md with beginner-friendly setup
  - Add docs/user-guide/troubleshooting.md with common issues and solutions

- **Documentation Infrastructure:**
  - Add docs/TODO_DOCUMENTATION_INDEX.md for tracking documentation tasks
  - Remove outdated brand/README.md (consolidated into main docs)

### chore: rename LICENSE.md to LICENSE

- Rename license file from LICENSE.md to LICENSE for standard naming convention

### fix(landing): make hero icon block element for proper stacking

- Add `display: block` to hero icon to prevent inline alignment issues with status badge

### fix(landing): center status badge in hero section

- Add `justify-content: center` to status badge for proper alignment
- Change margin from `margin-bottom: 32px` to `margin: 0 auto 32px auto` for horizontal centering

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
