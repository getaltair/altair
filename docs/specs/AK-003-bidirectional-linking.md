# Feature AK-003: Bidirectional Linking and Backlinks

## What it does

Enables wiki-style bidirectional linking between notes using [[note title]] syntax, with automatic backlink detection and visual relationship mapping for connected knowledge management.

## User Journey

GIVEN a user is writing a note about "Flutter State Management"
WHEN they type [[Riverpod]] in the content
THEN a link is created to the Riverpod note and the current note appears in Riverpod's backlinks panel

## Functional Requirements

- [[note title]] syntax for creating links while typing
- Auto-completion dropdown for existing note titles
- Create new note if linked note doesn't exist
- Backlinks panel showing all notes linking to current note
- Link preview on hover
- Fuzzy matching for note titles
- Alias support for links [[display text|actual note]]
- Link renaming updates all references
- Orphaned notes detection (no links in/out)
- Link graph statistics
- Bulk link operations (find/replace)
- Export with resolved links

## UI/UX Requirements

### Components

```dart
// Linking components
LinkAutoCompleteDropdown
BacklinksPanel
LinkPreviewTooltip
LinkHighlighter
OrphanedNotesWidget
LinkGraphStats
LinkRenameDialog
BrokenLinkIndicator
LinkContextMenu
LinkedMentionsCard
```

### Visual Design

- **Layout:**
  - Backlinks panel: 300px sidebar right
  - Autocomplete: floating dropdown 280px wide
  - Link preview: 400x300px tooltip
  - Link stats: bottom status bar
  
- **Colors:**
  ```dart
  validLink: Color(0xFF3B82F6), // Blue for existing links
  pendingLink: Color(0xFFF59E0B), // Amber for non-existent
  brokenLink: Color(0xFFEF4444), // Red for broken
  backlinkHighlight: Color(0xFF10B981), // Green for backlinks
  linkHover: Color(0xFF6366F1), // Indigo on hover
  ```
  
- **Typography:**
  - Links: 16px underlined
  - Autocomplete items: 14px regular
  - Backlink titles: 14px semibold
  - Preview text: 13px regular
  
- **Iconography:**
  - Link: link icon 16px inline
  - Backlink: arrow-left icon 16px
  - Broken: broken-chain icon 16px
  - New note: plus-circle icon 14px
  
- **Borders/Shadows:**
  - Autocomplete: 3px black border
  - Preview tooltip: 4px shadow
  - Backlinks panel: 2px left border

### User Interactions

- **Input Methods:**
  - Type [[ to trigger autocomplete
  - Click to follow link
  - Right-click for context menu
  - Drag link to create connection
  
- **Keyboard Shortcuts:**
  - `[[`: Start link creation
  - `Tab`: Accept autocomplete
  - `Ctrl+Click`: Open in new tab
  - `Ctrl+Shift+L`: Show all links
  - `Alt+Click`: Preview without opening
  - `Ctrl+Shift+B`: Toggle backlinks panel
  
- **Gestures:**
  - Long press: Preview on mobile
  - Swipe left on link: Quick edit
  - Pinch: Zoom link graph
  
- **Feedback:**
  - Link color indicates status
  - Underline on valid links
  - Tooltip shows target note
  - Count badge on backlinks

### State Management

```dart
// Riverpod providers
final currentNoteLinksProvider = StateNotifierProvider<LinksNotifier, List<NoteLink>>(
  (ref) => LinksNotifier(),
);

final backlinksProvider = FutureProvider.family<List<Backlink>, String>((ref, noteId) async {
  return ref.read(linkServiceProvider).getBacklinks(noteId);
});

final linkAutoCompleteProvider = FutureProvider.family<List<NoteSuggestion>, String>((ref, query) async {
  return ref.read(noteServiceProvider).searchNotes(query, limit: 10);
});

final linkPreviewProvider = FutureProvider.family<NotePreview, String>((ref, noteId) async {
  return ref.read(noteServiceProvider).getPreview(noteId);
});

final orphanedNotesProvider = FutureProvider<List<Note>>((ref) async {
  return ref.read(linkServiceProvider).getOrphanedNotes();
});

final linkGraphStatsProvider = StreamProvider<LinkGraphStats>((ref) {
  return ref.read(linkServiceProvider).statsStream;
});
```

### Responsive Behavior

- **Desktop (>1200px):**
  - Persistent backlinks sidebar
  - Full link preview tooltips
  - Multiple preview support
  
- **Tablet (768-1199px):**
  - Collapsible backlinks panel
  - Single preview at a time
  - Touch-optimized tooltips
  
- **Mobile (<768px):**
  - Bottom sheet for backlinks
  - Simplified previews
  - Link creation via toolbar

### Accessibility Requirements

- **Screen Reader:**
  - Link destination announced
  - Backlink count announced
  - Broken links identified
  
- **Keyboard Navigation:**
  - Tab through all links
  - Arrow keys in autocomplete
  - Enter to follow link
  
- **Color Contrast:**
  - Link colors meet WCAG AA
  - Alternative indicators for colorblind
  
- **Motion:**
  - Optional preview animations
  - Instant link following option
  
- **Font Sizing:**
  - Links scale with body text
  - Minimum touch target 44x44px

### ADHD-Specific UI Requirements

- **Cognitive Load:**
  - Max 5 autocomplete suggestions
  - Backlinks grouped by date/context
  - Progressive disclosure of link details
  - Clear visual link indicators
  
- **Focus Management:**
  - Auto-complete appears instantly
  - Preview doesn't steal focus
  - Persistent link highlighting
  
- **Forgiveness:**
  - Undo link creation
  - Easy link correction
  - Broken link recovery
  - Alias support for variations
  
- **Visual Hierarchy:**
  - Different colors for link states
  - Bold for frequently linked notes
  - Dimmed orphaned notes
  
- **Immediate Feedback:**
  - Instant autocomplete (<100ms)
  - Live link validation
  - Real-time backlink updates

## Non-Functional Requirements

### Performance Targets

- Autocomplete response: <50ms
- Link validation: <20ms
- Backlinks query: <200ms
- Preview load: <300ms
- Link rename cascade: <2s for 100 links

### Technical Constraints

- Flutter version: 3.16+
- Markdown link parsing
- SurrealDB graph queries
- Real-time link tracking
- Unicode support in titles

### Security Requirements

- Validate link targets
- Prevent circular references
- Sanitize link content
- Access control for linked notes

## Implementation Details

### Code Structure

```
lib/features/linking/
├── presentation/
│   ├── widgets/
│   │   ├── link_autocomplete_dropdown.dart
│   │   ├── backlinks_panel.dart
│   │   ├── link_preview_tooltip.dart
│   │   ├── link_highlighter.dart
│   │   └── orphaned_notes_widget.dart
│   ├── providers/
│   │   ├── links_provider.dart
│   │   ├── backlinks_provider.dart
│   │   └── autocomplete_provider.dart
│   └── screens/
│       └── link_graph_screen.dart
├── domain/
│   ├── models/
│   │   ├── note_link.dart
│   │   ├── backlink.dart
│   │   └── link_stats.dart
│   ├── repositories/
│   │   └── link_repository.dart
│   └── use_cases/
│       ├── create_link.dart
│       ├── get_backlinks.dart
│       └── rename_links.dart
└── data/
    ├── services/
    │   ├── link_parser_service.dart
    │   └── link_graph_service.dart
    └── repositories/
        └── link_repository_impl.dart
```

### Rust Backend Service

```rust
// rust-backend/services/linking/src/lib.rs
use axum::{Router, Json, Extension};
use surrealdb::Surreal;
use serde::{Deserialize, Serialize};
use regex::Regex;

#[derive(Serialize, Deserialize)]
struct NoteLink {
    from_note: String,
    to_note: String,
    link_text: String,
    position: usize,
    created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
struct Backlink {
    note_id: String,
    note_title: String,
    context: String,
    link_count: usize,
}

pub async fn extract_and_create_links(
    content: &str,
    note_id: &str,
    db: &Surreal<Client>,
) -> Result<Vec<NoteLink>, Error> {
    let link_regex = Regex::new(r"\[\[([^\]|]+)(?:\|([^\]]+))?\]\]").unwrap();
    let mut links = Vec::new();
    
    for cap in link_regex.captures_iter(content) {
        let link_text = cap.get(1).map_or("", |m| m.as_str());
        let display_text = cap.get(2).map(|m| m.as_str());
        
        // Find or create target note
        let target_note = find_or_create_note(link_text, db).await?;
        
        // Create link relationship
        let link = NoteLink {
            from_note: note_id.to_string(),
            to_note: target_note.id,
            link_text: display_text.unwrap_or(link_text).to_string(),
            position: cap.get(0).unwrap().start(),
            created_at: Utc::now(),
        };
        
        // Store link in database
        db.query("RELATE $from->links->$to SET link_text = $text, position = $pos")
            .bind(("from", &link.from_note))
            .bind(("to", &link.to_note))
            .bind(("text", &link.link_text))
            .bind(("pos", link.position))
            .await?;
        
        links.push(link);
    }
    
    Ok(links)
}

pub async fn get_backlinks(
    note_id: &str,
    db: &Surreal<Client>,
) -> Result<Vec<Backlink>, Error> {
    let backlinks: Vec<Backlink> = db
        .query("SELECT *, count() as link_count FROM notes WHERE ->links->$note_id GROUP BY id")
        .bind(("note_id", note_id))
        .await?;
    
    // Add context for each backlink
    for backlink in &mut backlinks {
        let context = extract_link_context(&backlink.note_id, note_id, db).await?;
        backlink.context = context;
    }
    
    Ok(backlinks)
}

pub async fn rename_note_links(
    old_title: &str,
    new_title: &str,
    db: &Surreal<Client>,
) -> Result<usize, Error> {
    // Find all notes containing links to old title
    let affected_notes: Vec<Note> = db
        .query("SELECT * FROM notes WHERE content CONTAINS $old_link")
        .bind(("old_link", format!("[[{}]]", old_title)))
        .await?;
    
    let mut updated_count = 0;
    for note in affected_notes {
        let updated_content = note.content.replace(
            &format!("[[{}]]", old_title),
            &format!("[[{}]]", new_title)
        );
        
        db.query("UPDATE notes SET content = $content WHERE id = $id")
            .bind(("content", updated_content))
            .bind(("id", note.id))
            .await?;
        
        updated_count += 1;
    }
    
    Ok(updated_count)
}
```

### gRPC Service Definition

```proto
// protos/linking.proto
syntax = "proto3";
package altair.knowledge.linking;

service LinkingService {
  rpc ExtractLinks(ExtractLinksRequest) returns (LinkList);
  rpc GetBacklinks(GetBacklinksRequest) returns (BacklinkList);
  rpc CreateLink(CreateLinkRequest) returns (NoteLink);
  rpc RenameLinks(RenameLinkRequest) returns (RenameResult);
  rpc GetOrphanedNotes(GetOrphanedRequest) returns (NoteList);
  rpc GetLinkGraph(GetGraphRequest) returns (LinkGraph);
  rpc ValidateLinks(ValidateLinksRequest) returns (ValidationResult);
}

message NoteLink {
  string from_note = 1;
  string to_note = 2;
  string link_text = 3;
  int32 position = 4;
  int64 created_at = 5;
  LinkStatus status = 6;
}

message Backlink {
  string note_id = 1;
  string note_title = 2;
  string context = 3;
  int32 link_count = 4;
  repeated string link_positions = 5;
}

message ExtractLinksRequest {
  string note_id = 1;
  string content = 2;
  bool create_missing = 3;
}

message GetBacklinksRequest {
  string note_id = 1;
  bool include_context = 2;
  int32 context_length = 3;
}

enum LinkStatus {
  VALID = 0;
  BROKEN = 1;
  PENDING = 2;
  CIRCULAR = 3;
}

message LinkGraph {
  repeated GraphNode nodes = 1;
  repeated GraphEdge edges = 2;
  LinkGraphStats stats = 3;
}

message LinkGraphStats {
  int32 total_notes = 1;
  int32 total_links = 2;
  int32 orphaned_notes = 3;
  float average_connections = 4;
  repeated string most_linked = 5;
}
```

### SurrealDB Schema

```sql
-- Note links relationship
DEFINE TABLE links SCHEMAFULL;
DEFINE FIELD in ON links TYPE record<notes>;
DEFINE FIELD out ON links TYPE record<notes>;
DEFINE FIELD link_text ON links TYPE string;
DEFINE FIELD position ON links TYPE int;
DEFINE FIELD created_at ON links TYPE datetime DEFAULT time::now();

-- Index for fast backlink queries
DEFINE INDEX links_out_idx ON links FIELDS out;
DEFINE INDEX links_in_idx ON links FIELDS in;

-- Function to get backlinks for a note
DEFINE FUNCTION fn::get_backlinks($note_id: string) {
  RETURN SELECT 
    in.id as note_id,
    in.title as note_title,
    link_text,
    count() as link_count
  FROM links
  WHERE out = $note_id
  GROUP BY in.id;
};

-- Function to find orphaned notes
DEFINE FUNCTION fn::find_orphaned_notes() {
  RETURN SELECT * FROM notes
  WHERE id NOT IN (
    SELECT DISTINCT in FROM links
    UNION
    SELECT DISTINCT out FROM links
  );
};

-- Function to get link graph stats
DEFINE FUNCTION fn::get_link_graph_stats() {
  LET $total_notes = (SELECT count() FROM notes);
  LET $total_links = (SELECT count() FROM links);
  LET $orphaned = (SELECT count() FROM fn::find_orphaned_notes());
  
  RETURN {
    total_notes: $total_notes,
    total_links: $total_links,
    orphaned_notes: $orphaned,
    average_connections: $total_links / $total_notes,
    most_linked: (
      SELECT out.title, count() as link_count
      FROM links
      GROUP BY out
      ORDER BY link_count DESC
      LIMIT 10
    )
  };
};
```

### Link Parser Implementation

```dart
// lib/features/linking/data/services/link_parser_service.dart
class LinkParserService {
  static final _linkRegex = RegExp(r'\[\[([^\]|]+)(?:\|([^\]]+))?\]\]');
  
  List<ParsedLink> parseLinks(String content) {
    final links = <ParsedLink>[];
    
    for (final match in _linkRegex.allMatches(content)) {
      final targetNote = match.group(1)!;
      final displayText = match.group(2);
      
      links.add(ParsedLink(
        targetNote: targetNote,
        displayText: displayText ?? targetNote,
        position: match.start,
        length: match.end - match.start,
      ));
    }
    
    return links;
  }
  
  String createLink(String noteTitle, {String? displayText}) {
    if (displayText != null && displayText != noteTitle) {
      return '[[$noteTitle|$displayText]]';
    }
    return '[[$noteTitle]]';
  }
  
  String updateLinksInContent(String content, String oldTitle, String newTitle) {
    // Handle both simple and aliased links
    final simplePattern = RegExp(r'\[\[${RegExp.escape(oldTitle)}\]\]');
    final aliasedPattern = RegExp(r'\[\[${RegExp.escape(oldTitle)}\|([^\]]+)\]\]');
    
    content = content.replaceAll(simplePattern, '[[$newTitle]]');
    content = content.replaceAllMapped(
      aliasedPattern,
      (match) => '[[$newTitle|${match.group(1)}]]',
    );
    
    return content;
  }
}
```

## Testing Requirements

### Unit Tests

```dart
// test/features/linking/domain/use_cases/create_link_test.dart
void main() {
  group('CreateLink', () {
    test('creates bidirectional link', () async {
      final useCase = CreateLink(mockRepo);
      final link = await useCase('note1', 'note2', 'Link Text');
      expect(link.fromNote, equals('note1'));
      expect(link.toNote, equals('note2'));
    });
    
    test('handles circular references', () async {
      // Test circular reference detection
    });
  });
}
```

### Widget Tests

```dart
// test/features/linking/presentation/widgets/link_autocomplete_test.dart
void main() {
  testWidgets('Autocomplete shows on [[ input', (tester) async {
    await tester.pumpWidget(LinkAutoCompleteDropdown());
    await tester.enterText(find.byType(TextField), '[[Flu');
    await tester.pumpAndSettle();
    expect(find.text('Flutter'), findsOneWidget);
  });
}
```

### Integration Tests

```dart
// integration_test/linking_flow_test.dart
void main() {
  testWidgets('Complete linking workflow', (tester) async {
    await tester.pumpWidget(MyApp());
    
    // Create a link
    await tester.enterText(find.byType(NoteEditor), 'Check out [[Flutter Guide]]');
    await tester.pumpAndSettle();
    
    // Verify link created
    expect(find.byType(LinkWidget), findsOneWidget);
    
    // Open backlinks panel
    await tester.tap(find.byIcon(Icons.link));
    await tester.pumpAndSettle();
    
    // Navigate to linked note
    await tester.tap(find.text('Flutter Guide'));
    await tester.pumpAndSettle();
    
    // Verify backlink appears
    expect(find.text('Linked from:'), findsOneWidget);
  });
}
```

### Accessibility Tests

- [ ] Link destinations announced
- [ ] Autocomplete navigation works
- [ ] Backlinks panel keyboard accessible
- [ ] Color contrast meets standards

## Definition of Done

- [ ] [[note]] syntax creates links
- [ ] Autocomplete works with fuzzy matching
- [ ] Backlinks update in real-time
- [ ] Link preview shows on hover
- [ ] Orphaned notes are identified
- [ ] Link renaming cascades properly
- [ ] All tests passing (>80% coverage)
- [ ] Accessibility audit complete
- [ ] Performance targets met
- [ ] Documentation updated
- [ ] Code review approved
