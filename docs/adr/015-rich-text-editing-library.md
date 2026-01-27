# ADR-015: Rich Text Editing Library Selection

| Field             | Value                                                      |
| ----------------- | ---------------------------------------------------------- |
| **Status**        | Proposed                                                   |
| **Date**          | 2026-01-14                                                 |
| **Deciders**      | Robert                                                     |
| **Supersedes**    | —                                                          |
| **Superseded by** | —                                                          |

## Context

Altair requires rich text editing capabilities across multiple modules:

- **Knowledge**: The primary use case—an Obsidian-like note-taking experience with markdown-native editing, live preview/WYSIWYG behavior, wiki-style linking (`[[note-name]]`), and support for formatting (bold, italic, headings, lists, code blocks, links).
- **Guidance**: Task descriptions and project documentation may benefit from basic rich text formatting.
- **Tracking**: Item descriptions and notes could use lightweight formatting.

The ecosystem for rich text editing in Kotlin/Compose is less mature than web-based alternatives (TipTap, ProseMirror, Slate). Key constraints include:

1. **Compose Multiplatform support**: Ideally a single library works across Android, iOS, and Desktop. Failing that, platform-specific solutions that share a common abstraction.
2. **Markdown interoperability**: Notes should be stored as markdown for portability and user data ownership.
3. **ADHD-friendly simplicity**: The editor should be distraction-free and keyboard-navigable, not overloaded with toolbars and options.

No native Kotlin/Compose block-based editor (equivalent to Notion or TipTap's block structure) currently exists. True block-based editing would require either custom implementation or WebView embedding of web editors.

## Decision

Adopt **compose-rich-editor** (`com.mohamedrejeb.richeditor:richeditor-compose`) as the platform-wide rich text editing foundation for Altair.

Supplement with:
- **multiplatform-markdown-renderer** for read-only markdown rendering (preview panes, displaying formatted content)
- **intellij-markdown** for custom markdown parsing needs (wiki-link syntax, custom extensions)
- **Custom Command Pattern implementation** for undo/redo functionality

For the Knowledge module specifically, implement a **split-pane or toggle-able preview** architecture rather than attempting true WYSIWYG-while-typing (which would require significant custom development for live markdown syntax transformation).

## Consequences

### Positive

- **True Compose Multiplatform support**: Single codebase works across Android, iOS, Desktop, and Web (WASM)
- **Active maintenance**: 1,700+ GitHub stars, regular releases (v1.0.0-rc13 as of June 2025), responsive maintainer
- **Markdown import/export built-in**: `setMarkdown()` and `toMarkdown()` methods align with our data portability goals
- **Idiomatic Compose API**: `RichTextState` follows familiar state management patterns
- **No WebView overhead**: Native performance, lower memory usage, faster startup—critical for mobile
- **Consistent UX across platforms**: Same editing behavior everywhere vs. platform-specific quirks

### Negative

- **Not block-based**: Cannot achieve Notion-style drag-and-drop block reordering without significant custom work
- **No live markdown syntax highlighting**: Users won't see markdown symbols transform as they type (like Obsidian's live preview mode)
- **Limited table support**: Tables require HTML import rather than markdown table syntax
- **No built-in toolbar**: Must implement our own formatting UI (though this aligns with ADHD-friendly minimalism goals)
- **RC status**: Library is not yet at 1.0 stable, though API has been stable for several releases
- **Wiki-link syntax requires custom parsing**: `[[link]]` syntax not supported out of the box

### Neutral

- Code block entry via triple-backtick typing is not supported; formatting must be applied via API/toolbar
- Images are block-level only, not inline
- The library handles basic rich text well but complex document structures may push its limits

## Alternatives Considered

### Alternative 1: WebView-embedded TipTap/ProseMirror

Embed a web-based editor (TipTap, ProseMirror, or BlockNote) via WebView using compose-webview-multiplatform or JxBrowser.

**Rejected because**:
- Adds significant complexity (JavaScript bridge communication, state synchronization)
- Higher memory usage and slower startup, particularly impactful on mobile
- Worse offline behavior and potential keystroke latency
- Contradicts our mobile-first rebalancing—WebView editing is notoriously problematic on mobile
- Overkill for our requirements; we don't need real-time collaboration or complex embeds

### Alternative 2: halilozercan/compose-richtext

A modular rich text library from a Google Android Text Team engineer with markdown rendering support.

**Rejected because**:
- Explicitly marked as "very experimental" with an "unclear roadmap"
- No iOS support, breaking our multiplatform requirements
- Primarily focused on rendering, not editing

### Alternative 3: canopas/rich-editor-compose

Android WYSIWYG editor for Jetpack Compose.

**Rejected because**:
- Android-only (no multiplatform support)
- Appears stagnant (only 16 commits, last activity November 2023)
- Limited community adoption (91 stars)

### Alternative 4: Custom implementation from scratch

Build a custom rich text editor using BasicTextField/TextFieldState and AnnotatedString.

**Rejected because**:
- Massive engineering effort for functionality that compose-rich-editor already provides
- Would delay Knowledge module delivery significantly
- Text editing edge cases (cursor positioning, selection, IME handling) are notoriously difficult
- Better to extend a working foundation than start from zero

### Alternative 5: Platform-specific native editors

Use platform-native text editing (Android EditText with spans, iOS UITextView with NSAttributedString, Desktop with Swing/AWT) wrapped in Compose.

**Rejected because**:
- Three separate implementations to maintain
- Inconsistent behavior and feature sets across platforms
- Compose interop adds complexity without clear benefit over compose-rich-editor
- Harder to implement shared features like wiki-linking consistently

## References

- [compose-rich-editor GitHub](https://github.com/MohamedRejeb/compose-rich-editor)
- [multiplatform-markdown-renderer GitHub](https://github.com/mikepenz/multiplatform-markdown-renderer)
- [intellij-markdown GitHub](https://github.com/JetBrains/markdown)
- [Compose Multiplatform 1.7 Release Notes](https://github.com/JetBrains/compose-multiplatform/releases/tag/v1.7.0)
- Research: Kotlin Compose Rich Text Editors for Obsidian-Style Note-Taking Apps (2026-01-14)
