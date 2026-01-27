# ADR-014: Source Document Architecture

| Field             | Value           |
| ----------------- | --------------- |
| **Status**        | Accepted        |
| **Date**          | 2026-01-14      |
| **Deciders**      | Robert Hamilton |
| **Supersedes**    | —               |
| **Superseded by** | —               |

## Context

Users want to integrate external documents (PDFs, Markdown files, etc.) into their Knowledge base.
Common use cases:
- Import research papers for annotation
- Watch a folder of documentation (like Altair's own PRDs)
- Make ebooks searchable alongside personal notes
- Link notes to specific sections of imported documents

We faced a fundamental design tension:

**Editable Notes**: User can annotate, enhance, correct → but content diverges from source
**Linked References**: Source stays authoritative → but where do user annotations live?

For watched files especially (like your PRD docs), we needed:
1. Content searchable in Knowledge
2. Content visible in graph relationships
3. Source file remains authoritative (external edits reflected)
4. User can add annotations about the content

## Decision

We chose a **hybrid model with SourceDocuments and Annotations** — imported files are read-only
references, users annotate them with separate annotation entities, and Notes can link to both.

### Architecture

```
SourceDocument (read-only imported content)
├── extracted_text (searchable)
├── embedding (semantic search)
├── content_hash (detect changes)
└── status (pending, processed, stale, failed)

SourceAnnotation (user's thoughts anchored to document)
├── anchor_type (document, page, heading, selection)
├── anchor_fingerprint (SimHash for fuzzy matching)
└── content (user's annotation, Markdown)

WatchedFolder (auto-discovery configuration)
├── path, include/exclude patterns
├── scan_interval
└── initiative_id (auto-link new docs)

ExtractionJob (server-side processing queue)
├── source_document_id
├── status (queued, processing, completed, failed)
└── error_message
```

### Key Design Decisions

1. **SourceDocuments are read-only**: User cannot edit extracted content
2. **Annotations are separate entities**: Anchored to locations, survive document changes
3. **SimHash fingerprinting**: Fuzzy anchor resolution when content moves
4. **Server-side extraction**: Compute-heavy processing offloaded to server
5. **Watched folders**: Auto-import from file system with change detection
6. **Note linking syntax**: `[[Source:filename.pdf]]` and `[[Source:filename.pdf#heading]]`

## Alternatives Considered

### Notes with Attached Files

Import creates a Note with file as attachment; user edits the Note freely:

**Rejected because:**
- Content diverges from source
- Doesn't handle watched files (would need to re-import on every change)
- Loses ability to search original content specifically
- Mixing authored vs. imported content

### Full-Text Indexing Only

Import just indexes content for search; no separate entity:

**Rejected because:**
- Cannot annotate specific sections
- No graph representation
- No tracked relationship between imports
- Cannot distinguish imported vs. authored content

### Chunked Notes (One Note per Section)

Import creates multiple Notes, one per heading/section:

**Rejected because:**
- Creates "noise" in Note list
- Section boundaries may not match user mental model
- Updating source requires recreating all chunks
- Loses document-level context

## Consequences

### Positive

- **Source stays authoritative**: External edits reflected on re-extraction
- **Structured annotations**: User thoughts anchored to specific locations
- **Searchable**: Both extracted text and annotations included in search
- **Graph integration**: SourceDocuments appear as nodes with distinct style
- **Watched folders**: Documentation automatically stays current
- **Initiative integration**: Link entire folders to projects

### Negative

- **Complexity**: Multiple new entities (SourceDocument, SourceAnnotation, WatchedFolder, ExtractionJob)
- **Anchor drift**: Annotations may lose their anchor when content changes significantly
- **Storage overhead**: Extracted text stored alongside source
- **Server dependency**: Extraction requires server for compute

### Mitigations

- **SimHash fuzzy matching**: Anchors survive moderate content changes
- **Fallback to document level**: Orphaned anchors still visible
- **User prompt on conflict**: "Section may have moved" indicator
- **Offline viewing**: Extracted text available offline after initial extraction

## Implementation Notes

### SourceDocument Entity

```kotlin
data class SourceDocument(
    val id: String,
    val userId: String,
    val title: String,
    val sourceType: SourceType,
    val sourcePath: String,
    val mimeType: String,
    val contentHash: String,
    val extractedText: String?,
    val embedding: FloatArray?,
    val status: ExtractionStatus,
    val errorMessage: String?,
    val initiativeId: String?,
    val watchedFolderId: String?,
    val lastSyncedAt: Instant?,
    val createdAt: Instant,
    val updatedAt: Instant
)

enum class SourceType { FILE, URI, WATCHED }
enum class ExtractionStatus { PENDING, PROCESSED, FAILED, STALE }
```

### Anchor Fingerprinting

SimHash creates a fingerprint that's similar for similar content:

```kotlin
data class AnchorFingerprint(
    val headingText: String?,
    val headingSimhash: String?,
    val contentSimhash: String?,
    val structuralHint: String?
)

// Resolution order:
// 1. Exact heading match → high confidence
// 2. SimHash heading (Hamming < 3) → high (renamed)
// 3. SimHash content (Hamming < 5) → medium (moved)
// 4. Structural hint + similarity → medium
// 5. Document level → show "may have moved"
```

### Extraction Pipeline

```
1. SourceDocument created (status: pending)
2. ExtractionJob queued
3. Server processes:
   - PDF: PyMuPDF or similar
   - Markdown: Parse headings, preserve structure
   - Plain text: Direct extraction
4. Update SourceDocument with extracted_text, embedding
5. Status → processed
```

### WatchedFolder Scanner

```kotlin
class WatchedFolderScanner {
    suspend fun scan(folder: WatchedFolder) {
        val files = listFiles(folder.path, folder.includePatterns, folder.excludePatterns)
        
        for (file in files) {
            val hash = sha256(file)
            val existing = sourceDocumentRepository.findByPath(file.path)
            
            when {
                existing == null -> {
                    // New file
                    createSourceDocument(file, folder)
                }
                existing.contentHash != hash -> {
                    // Modified file
                    markStaleAndRequeue(existing)
                }
                // Unchanged: skip
            }
        }
        
        // Handle deleted files
        val deletedDocs = findDeletedInFolder(folder)
        deletedDocs.forEach { softDelete(it) }
    }
}
```

### Universal Inbox Integration

File uploads can go through Inbox:

```kotlin
// Triage options for file attachments:
sealed class FileTriageAction {
    data class CreateNoteWithAttachment(val folderId: String?) : FileTriageAction()
    data class ImportAsSourceDocument(val initiativeId: String?) : FileTriageAction()
}
```

### v1 vs Future Scope

| Feature | v1 | v1.1+ |
|---------|-----|-------|
| Document-level embedding | ✓ | Chunk-level |
| Heading-level anchors | ✓ | Selection-level |
| PDF, Markdown, plain text | ✓ | EPUB, DOCX, HTML |
| Manual file import | ✓ | - |
| Watched folders | ✓ | Cloud folder sync |
| Sidebar annotations | ✓ | Inline highlights |
| Annotation → Note promotion | ✓ | - |

### Annotation UI

v1 uses a sidebar panel approach:
- List of annotations for current document
- Click annotation → scroll to anchor location
- "Section may have moved" indicator for low-confidence anchors
- "Promote to Note" action creates linked Note

### Deletion Behavior

When user deletes a SourceDocument:
1. Prompt: "Delete annotations too, keep orphaned, or cancel?"
2. Optional: "Remember my choice" checkbox
3. Soft-delete SourceDocument
4. Handle annotations per user choice

## References

- [altair-prd-knowledge.md](../requirements/altair-prd-knowledge.md) — Knowledge module requirements
- [domain-model.md](../architecture/domain-model.md) — SourceDocument, SourceAnnotation entities
- [event-bus.md](../architecture/event-bus.md) — SourceDocumentEvent types
- [persistence.md](../architecture/persistence.md) — Database schemas

---

_Decision made: 2026-01-14_
