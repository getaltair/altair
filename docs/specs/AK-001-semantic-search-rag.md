# Feature AK-001: Semantic Search and RAG

## What it does

Provides intelligent search across all notes using local embeddings and vector similarity, with source-grounded results and fallback to keyword search for comprehensive information retrieval.

## User Journey

GIVEN a user has multiple notes stored in Altair Knowledge
WHEN they search for conceptually related information
THEN they receive semantically relevant results ranked by similarity with highlighted source citations

## Functional Requirements

- Local embedding generation using Sentence Transformers (all-MiniLM-L6-v2)
- Vector storage in SurrealDB with cosine similarity search
- Hybrid search combining semantic + keyword matching
- Source citation with note titles and relevant excerpts
- Real-time indexing of new/modified notes
- Fallback to keyword search when semantic search unavailable
- Search history and recent queries
- Search filters (date range, tags, note type)
- Export search results to markdown

## UI/UX Requirements

### Components

```dart
// Main search components
SearchBar
SearchResultsList
SearchFilterPanel
SemanticScoreIndicator
SourceCitationCard
SearchHistoryDropdown
QuickSearchWidget
AdvancedSearchDialog
```

### Visual Design

- **Layout:** 
  - Top search bar with 48px height
  - Two-column layout: results (70%) + filters (30%)
  - 16px padding, 8px gap between results
  - Sticky search bar on scroll
  
- **Colors:**
  ```dart
  primarySearch: Color(0xFF6366F1), // Indigo for search button
  semanticMatch: Color(0xFF10B981), // Green for high relevance
  keywordMatch: Color(0xFFF59E0B), // Amber for keyword matches
  filterActive: Color(0xFF3B82F6), // Blue for active filters
  ```
  
- **Typography:**
  - Search input: 16px regular
  - Result titles: 18px semibold
  - Excerpts: 14px regular with 1.5 line height
  - Relevance scores: 12px monospace
  
- **Iconography:**
  - Search: magnifying glass 24px
  - Semantic: brain icon 20px
  - Keyword: tag icon 20px
  - Filter: funnel icon 20px
  
- **Borders/Shadows:**
  - Neo-brutalist: 3px black borders
  - Shadow offset: 4px bottom-right
  - Active state: 6px shadow offset

### User Interactions

- **Input Methods:**
  - Keyboard: Type to search, Enter to execute
  - Voice: Mic button for voice search
  - Paste: Ctrl+V for multi-line queries
  
- **Keyboard Shortcuts:**
  - `Ctrl+K`: Focus search bar
  - `Ctrl+Shift+F`: Advanced search
  - `Escape`: Clear search
  - `Tab`: Navigate results
  - `Enter`: Open selected result
  
- **Gestures:**
  - Pull-to-refresh results
  - Swipe right to bookmark
  - Long-press for preview
  
- **Feedback:**
  - Loading spinner during search
  - "No results" with suggestions
  - Match highlighting in excerpts
  - Relevance score tooltips

### State Management

```dart
// Riverpod providers
final searchQueryProvider = StateProvider<String>((ref) => '');

final searchFiltersProvider = StateNotifierProvider<SearchFiltersNotifier, SearchFilters>(
  (ref) => SearchFiltersNotifier(),
);

final searchResultsProvider = FutureProvider<List<SearchResult>>((ref) async {
  final query = ref.watch(searchQueryProvider);
  final filters = ref.watch(searchFiltersProvider);
  return ref.read(searchServiceProvider).search(query, filters);
});

final searchHistoryProvider = StateNotifierProvider<SearchHistoryNotifier, List<String>>(
  (ref) => SearchHistoryNotifier(),
);

final embeddingStatusProvider = StreamProvider<EmbeddingStatus>((ref) {
  return ref.read(embeddingServiceProvider).statusStream;
});
```

### Responsive Behavior

- **Desktop (>1200px):**
  - Two-column layout with sidebar filters
  - Multiple results visible (8-10)
  - Hover previews enabled
  
- **Tablet (768-1199px):**
  - Single column with collapsible filters
  - 5-7 results visible
  - Touch-optimized interactions
  
- **Mobile (<768px):**
  - Full-width search bar
  - Bottom sheet for filters
  - 3-4 results visible
  - Swipe gestures enabled

### Accessibility Requirements

- **Screen Reader:**
  - ARIA labels for all interactive elements
  - Results announced with relevance scores
  - Filter changes announced
  
- **Keyboard Navigation:**
  - Full keyboard control
  - Visible focus indicators
  - Logical tab order
  
- **Color Contrast:**
  - WCAG 2.1 AA minimum
  - Alternative relevance indicators (icons)
  
- **Motion:**
  - Respect prefers-reduced-motion
  - Optional animations
  
- **Font Sizing:**
  - Minimum 14px for body text
  - Scalable with system settings

### ADHD-Specific UI Requirements

- **Cognitive Load:**
  - Max 10 results initially (load more on demand)
  - Progressive disclosure of filters
  - Simple/Advanced mode toggle
  - Clear primary action (search button)
  
- **Focus Management:**
  - Auto-focus search on page load
  - Persistent search context
  - Visual breadcrumbs for navigation
  
- **Forgiveness:**
  - Search history for easy re-runs
  - "Did you mean?" suggestions
  - Clear all filters button
  
- **Visual Hierarchy:**
  - Color-coded relevance scores
  - Bold keyword highlights
  - Prominent result titles
  
- **Immediate Feedback:**
  - Instant search suggestions
  - Live result count
  - <300ms search execution

## Non-Functional Requirements

### Performance Targets

- Embedding generation: <500ms per note
- Vector search: <100ms for 10k notes
- Full search execution: <1 second
- Result rendering: <200ms
- Index update: <2 seconds per note

### Technical Constraints

- Flutter version: 3.16+
- Rust backend with Axum
- SurrealDB 2.0+ for vector operations
- Python service for embeddings (or Rust equivalent)
- Max embedding dimensions: 384 (all-MiniLM-L6-v2)

### Security Requirements

- Local embeddings only (no external API by default)
- Sanitize search queries
- Rate limiting for embedding generation
- Secure storage of vectors

## Implementation Details

### Code Structure

```
lib/features/search/
├── presentation/
│   ├── widgets/
│   │   ├── search_bar.dart
│   │   ├── search_results_list.dart
│   │   ├── search_filter_panel.dart
│   │   └── semantic_score_indicator.dart
│   ├── providers/
│   │   ├── search_provider.dart
│   │   └── search_filters_provider.dart
│   └── screens/
│       └── search_screen.dart
├── domain/
│   ├── models/
│   │   ├── search_result.dart
│   │   └── search_filters.dart
│   ├── repositories/
│   │   └── search_repository.dart
│   └── use_cases/
│       ├── semantic_search.dart
│       └── keyword_search.dart
└── data/
    ├── services/
    │   ├── embedding_service.dart
    │   └── surrealdb_search_service.dart
    └── repositories/
        └── search_repository_impl.dart
```

### Rust Backend Service

```rust
// rust-backend/services/search/src/lib.rs
use axum::{Router, Json};
use surrealdb::Surreal;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct SearchRequest {
    query: String,
    filters: SearchFilters,
    limit: usize,
}

#[derive(Serialize)]
struct SearchResult {
    note_id: String,
    title: String,
    excerpt: String,
    relevance_score: f32,
    match_type: MatchType,
}

pub async fn search_handler(
    Json(req): Json<SearchRequest>,
    db: Extension<Surreal<Client>>,
) -> Result<Json<Vec<SearchResult>>, Error> {
    // Generate embedding for query
    let query_embedding = embedding_service.embed(&req.query).await?;
    
    // Perform vector search in SurrealDB
    let semantic_results = db
        .query("SELECT * FROM notes WHERE vector <|10,384|> $embedding")
        .bind(("embedding", query_embedding))
        .await?;
    
    // Combine with keyword search
    let keyword_results = db
        .query("SELECT * FROM notes WHERE content @@ $query")
        .bind(("query", req.query))
        .await?;
    
    // Merge and rank results
    let combined = merge_results(semantic_results, keyword_results);
    
    Ok(Json(combined))
}
```

### gRPC Service Definition

```proto
// protos/search.proto
syntax = "proto3";
package altair.knowledge.search;

service SearchService {
  rpc Search(SearchRequest) returns (SearchResponse);
  rpc GenerateEmbedding(EmbeddingRequest) returns (EmbeddingResponse);
  rpc IndexNote(IndexRequest) returns (IndexResponse);
  rpc UpdateIndex(UpdateIndexRequest) returns (UpdateIndexResponse);
}

message SearchRequest {
  string query = 1;
  SearchFilters filters = 2;
  int32 limit = 3;
  int32 offset = 4;
  SearchMode mode = 5;
}

message SearchResult {
  string note_id = 1;
  string title = 2;
  string excerpt = 3;
  float relevance_score = 4;
  MatchType match_type = 5;
  repeated string highlighted_terms = 6;
}

message SearchFilters {
  int64 date_from = 1;
  int64 date_to = 2;
  repeated string tags = 3;
  string note_type = 4;
}

enum SearchMode {
  HYBRID = 0;
  SEMANTIC_ONLY = 1;
  KEYWORD_ONLY = 2;
}

enum MatchType {
  SEMANTIC = 0;
  KEYWORD = 1;
  BOTH = 2;
}
```

### SurrealDB Schema

```sql
-- SurrealDB schema for search
DEFINE TABLE notes SCHEMAFULL;
DEFINE FIELD title ON notes TYPE string;
DEFINE FIELD content ON notes TYPE string;
DEFINE FIELD embedding ON notes TYPE array<float>;
DEFINE FIELD created_at ON notes TYPE datetime;
DEFINE FIELD updated_at ON notes TYPE datetime;
DEFINE FIELD tags ON notes TYPE array<string>;

-- Vector index for semantic search
DEFINE INDEX embedding_idx ON notes FIELDS embedding VECTOR DIMENSION 384;

-- Full-text search index
DEFINE INDEX content_idx ON notes FIELDS content SEARCH ANALYZER english;

-- Search function
DEFINE FUNCTION fn::semantic_search($query_embedding: array<float>, $limit: int) {
  RETURN SELECT 
    id,
    title,
    content,
    vector::similarity::cosine(embedding, $query_embedding) as score
  FROM notes
  ORDER BY score DESC
  LIMIT $limit;
};
```

## Testing Requirements

### Unit Tests

```dart
// test/features/search/domain/use_cases/semantic_search_test.dart
void main() {
  group('SemanticSearch', () {
    test('returns relevant results for query', () async {
      final search = SemanticSearch(mockRepo);
      final results = await search('flutter state management');
      expect(results.length, greaterThan(0));
      expect(results.first.relevanceScore, greaterThan(0.7));
    });
    
    test('falls back to keyword search on embedding failure', () async {
      // Test fallback behavior
    });
  });
}
```

### Widget Tests

```dart
// test/features/search/presentation/widgets/search_bar_test.dart
void main() {
  testWidgets('SearchBar updates query on input', (tester) async {
    await tester.pumpWidget(SearchBar());
    await tester.enterText(find.byType(TextField), 'test query');
    expect(find.text('test query'), findsOneWidget);
  });
}
```

### Integration Tests

```dart
// integration_test/search_flow_test.dart
void main() {
  testWidgets('Complete search flow', (tester) async {
    // Test end-to-end search functionality
    await tester.pumpWidget(MyApp());
    await tester.tap(find.byIcon(Icons.search));
    await tester.enterText(find.byType(TextField), 'flutter');
    await tester.tap(find.text('Search'));
    await tester.pumpAndSettle();
    expect(find.byType(SearchResultCard), findsWidgets);
  });
}
```

### Accessibility Tests

- [ ] Verify ARIA labels with screen reader
- [ ] Test keyboard-only navigation
- [ ] Validate color contrast ratios
- [ ] Check focus management

## Definition of Done

- [ ] Semantic search returns relevant results
- [ ] Keyword fallback works when embeddings unavailable
- [ ] Search executes in <1 second
- [ ] Results properly ranked by relevance
- [ ] Filters work correctly
- [ ] All tests passing (>80% coverage)
- [ ] Accessibility audit complete
- [ ] Performance benchmarks met
- [ ] Documentation updated
- [ ] Code review approved
