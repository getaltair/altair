# Feature AK-005: Knowledge Graph Visualization

## What it does

Provides an interactive graph visualization of note relationships, displaying bidirectional links as an explorable network with clustering, filtering, and navigation capabilities for understanding knowledge connections.

## User Journey

GIVEN a user has created multiple interconnected notes
WHEN they open the knowledge graph view
THEN they see a visual network of their notes with links as edges, can zoom/pan/filter, and click nodes to navigate

## Functional Requirements

- Interactive node-link graph visualization
- Real-time graph updates as links change
- Node sizing by connection count
- Edge styling for link types
- Clustering by topics/tags
- Filter by date range, tags, or search
- Zoom/pan navigation
- Node preview on hover
- Click to open note
- Graph statistics sidebar
- Export graph as image
- Multiple layout algorithms
- Path finding between notes
- Community detection

## UI/UX Requirements

### Components

```dart
// Graph visualization components
GraphCanvas
GraphNode
GraphEdge
GraphControls
GraphFilterPanel
GraphStatsPanel
GraphMinimap
NodeTooltip
GraphLayoutSelector
GraphExportDialog
PathFinderWidget
ClusterIndicator
```

### Visual Design

- **Layout:**
  - Full-screen canvas with overlay controls
  - Right sidebar for filters (300px)
  - Bottom stats bar (48px)
  - Floating minimap (200x150px)
  - Top toolbar (56px)
  
- **Colors:**
  ```dart
  nodeDefault: Color(0xFF6366F1), // Indigo nodes
  nodeHover: Color(0xFF4F46E5), // Darker on hover
  nodeActive: Color(0xFF10B981), // Green for current
  edgeDefault: Color(0xFFD1D5DB), // Gray edges
  edgeHighlight: Color(0xFF3B82F6), // Blue for path
  clusterColor: Color(0xFFF3F4F6), // Light gray clusters
  ```
  
- **Typography:**
  - Node labels: 12px semibold
  - Edge labels: 10px regular
  - Stats text: 14px monospace
  - Tooltip: 14px regular
  
- **Iconography:**
  - Zoom in/out: magnifier icons 20px
  - Reset view: home icon 20px
  - Filter: funnel icon 18px
  - Layout: grid icon 18px
  
- **Borders/Shadows:**
  - Nodes: 2px border, 4px shadow
  - Active node: 8px glow effect
  - Clusters: dashed 2px border

### User Interactions

- **Input Methods:**
  - Mouse: Click, drag, scroll wheel zoom
  - Touch: Pinch zoom, pan, tap
  - Keyboard: Arrow key navigation
  - Search: Type to filter nodes
  
- **Keyboard Shortcuts:**
  - `Ctrl+G`: Open graph view
  - `+/-`: Zoom in/out
  - `0`: Reset zoom
  - `F`: Fit to screen
  - `Arrows`: Navigate nodes
  - `Enter`: Open selected node
  - `Space`: Center on node
  
- **Gestures:**
  - Pinch: Zoom
  - Two-finger drag: Pan
  - Double-tap: Focus node
  - Long-press: Node menu
  
- **Feedback:**
  - Smooth animations (60fps)
  - Node highlight on hover
  - Edge glow on traverse
  - Loading spinner for large graphs

### State Management

```dart
// Riverpod providers
final graphDataProvider = FutureProvider<GraphData>((ref) async {
  return ref.read(graphServiceProvider).buildGraph();
});

final graphLayoutProvider = StateProvider<GraphLayout>((ref) => GraphLayout.forceDirected);

final graphFiltersProvider = StateNotifierProvider<GraphFiltersNotifier, GraphFilters>(
  (ref) => GraphFiltersNotifier(),
);

final selectedNodeProvider = StateProvider<String?>((ref) => null);

final graphViewportProvider = StateNotifierProvider<ViewportNotifier, GraphViewport>(
  (ref) => ViewportNotifier(),
);

final graphStatsProvider = Provider<GraphStats>((ref) {
  final graph = ref.watch(graphDataProvider).value;
  return graph != null ? calculateStats(graph) : GraphStats.empty();
});

final nodePathProvider = FutureProvider.family<List<String>, PathQuery>((ref, query) async {
  final graph = ref.watch(graphDataProvider).value;
  return findShortestPath(graph, query.from, query.to);
});
```

### Responsive Behavior

- **Desktop (>1200px):**
  - Full graph with all controls
  - Multi-select with Ctrl
  - Detailed node previews
  
- **Tablet (768-1199px):**
  - Simplified controls
  - Touch-optimized
  - Single selection only
  
- **Mobile (<768px):**
  - Minimal UI overlay
  - Gesture-based navigation
  - List view fallback option

### Accessibility Requirements

- **Screen Reader:**
  - Graph structure described
  - Node relationships announced
  - Navigation instructions
  
- **Keyboard Navigation:**
  - Full keyboard control
  - Tab through nodes
  - Clear focus indicators
  
- **Color Contrast:**
  - High contrast mode
  - Pattern fills option
  - Size-based distinction
  
- **Motion:**
  - Reduced motion option
  - Instant transitions
  - Static layout option
  
- **Font Sizing:**
  - Zoomable labels
  - Minimum readable size

### ADHD-Specific UI Requirements

- **Cognitive Load:**
  - Progressive disclosure of nodes
  - Start with immediate connections
  - Expand on demand
  - Hide distant nodes option
  
- **Focus Management:**
  - Highlight current context
  - Dim unrelated nodes
  - Path highlighting
  - Breadcrumb trail
  
- **Forgiveness:**
  - Undo navigation
  - Reset view button
  - Save/restore positions
  - Multiple layout attempts
  
- **Visual Hierarchy:**
  - Size indicates importance
  - Color for categories
  - Proximity for relevance
  - Animation for attention
  
- **Immediate Feedback:**
  - Instant node response
  - Smooth transitions
  - Live filter updates
  - Quick reset options

## Non-Functional Requirements

### Performance Targets

- Render 1000 nodes: <1 second
- Smooth pan/zoom: 60fps
- Layout calculation: <3 seconds
- Filter application: <200ms
- Node interaction: <50ms response

### Technical Constraints

- Flutter version: 3.16+
- Canvas/WebGL rendering
- Graph layout algorithms (D3-style)
- Quadtree for collision detection
- Worker isolates for layout

### Security Requirements

- Sanitize node labels
- Prevent XSS in tooltips
- Secure export functionality
- Access control for shared graphs

## Implementation Details

### Code Structure

```
lib/features/knowledge_graph/
├── presentation/
│   ├── widgets/
│   │   ├── graph_canvas.dart
│   │   ├── graph_node.dart
│   │   ├── graph_edge.dart
│   │   ├── graph_controls.dart
│   │   └── graph_minimap.dart
│   ├── providers/
│   │   ├── graph_data_provider.dart
│   │   ├── graph_layout_provider.dart
│   │   └── graph_filters_provider.dart
│   └── screens/
│       └── knowledge_graph_screen.dart
├── domain/
│   ├── models/
│   │   ├── graph_data.dart
│   │   ├── graph_node.dart
│   │   └── graph_stats.dart
│   ├── algorithms/
│   │   ├── force_directed.dart
│   │   ├── hierarchical.dart
│   │   └── community_detection.dart
│   └── use_cases/
│       ├── build_graph.dart
│       └── find_paths.dart
└── data/
    ├── services/
    │   └── graph_service.dart
    └── repositories/
        └── graph_repository_impl.dart
```

### Rust Backend Service

```rust
// rust-backend/services/graph/src/lib.rs
use axum::{Router, Json, Extension};
use surrealdb::Surreal;
use petgraph::graph::{Graph, NodeIndex};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct GraphData {
    nodes: Vec<GraphNode>,
    edges: Vec<GraphEdge>,
    clusters: Vec<Cluster>,
    stats: GraphStats,
}

#[derive(Serialize, Deserialize)]
struct GraphNode {
    id: String,
    label: String,
    x: f32,
    y: f32,
    size: f32,
    color: String,
    cluster: Option<String>,
    metadata: NodeMetadata,
}

#[derive(Serialize, Deserialize)]
struct GraphEdge {
    source: String,
    target: String,
    weight: f32,
    edge_type: EdgeType,
}

pub async fn build_knowledge_graph(
    db: Extension<Surreal<Client>>,
) -> Result<Json<GraphData>, Error> {
    // Fetch all notes and links
    let notes: Vec<Note> = db.select("notes").await?;
    let links: Vec<Link> = db.select("links").await?;
    
    // Build petgraph
    let mut graph = Graph::<Note, Link>::new();
    let mut node_indices = HashMap::new();
    
    // Add nodes
    for note in &notes {
        let idx = graph.add_node(note.clone());
        node_indices.insert(note.id.clone(), idx);
    }
    
    // Add edges
    for link in &links {
        if let (Some(&from), Some(&to)) = (
            node_indices.get(&link.from_note),
            node_indices.get(&link.to_note),
        ) {
            graph.add_edge(from, to, link.clone());
        }
    }
    
    // Apply layout algorithm
    let layout = calculate_force_directed_layout(&graph);
    
    // Detect communities
    let communities = detect_communities(&graph);
    
    // Calculate stats
    let stats = calculate_graph_stats(&graph);
    
    // Convert to frontend format
    let graph_data = GraphData {
        nodes: build_nodes(&graph, &layout, &communities),
        edges: build_edges(&graph),
        clusters: communities,
        stats,
    };
    
    Ok(Json(graph_data))
}

fn calculate_force_directed_layout(graph: &Graph<Note, Link>) -> HashMap<NodeIndex, (f32, f32)> {
    let mut positions = HashMap::new();
    let mut velocities = HashMap::new();
    
    // Initialize random positions
    for node in graph.node_indices() {
        positions.insert(node, (rand::random::<f32>() * 1000.0, rand::random::<f32>() * 1000.0));
        velocities.insert(node, (0.0, 0.0));
    }
    
    // Run force simulation
    const ITERATIONS: usize = 100;
    const REPULSION: f32 = 100.0;
    const ATTRACTION: f32 = 0.1;
    const DAMPING: f32 = 0.9;
    
    for _ in 0..ITERATIONS {
        // Calculate repulsive forces between all nodes
        for node1 in graph.node_indices() {
            let pos1 = positions[&node1];
            let mut force = (0.0, 0.0);
            
            for node2 in graph.node_indices() {
                if node1 != node2 {
                    let pos2 = positions[&node2];
                    let dx = pos1.0 - pos2.0;
                    let dy = pos1.1 - pos2.1;
                    let dist_sq = dx * dx + dy * dy + 0.01;
                    
                    force.0 += REPULSION * dx / dist_sq;
                    force.1 += REPULSION * dy / dist_sq;
                }
            }
            
            velocities.get_mut(&node1).unwrap().0 += force.0;
            velocities.get_mut(&node1).unwrap().1 += force.1;
        }
        
        // Calculate attractive forces along edges
        for edge in graph.edge_indices() {
            let (source, target) = graph.edge_endpoints(edge).unwrap();
            let pos1 = positions[&source];
            let pos2 = positions[&target];
            
            let dx = pos2.0 - pos1.0;
            let dy = pos2.1 - pos1.1;
            
            velocities.get_mut(&source).unwrap().0 += ATTRACTION * dx;
            velocities.get_mut(&source).unwrap().1 += ATTRACTION * dy;
            velocities.get_mut(&target).unwrap().0 -= ATTRACTION * dx;
            velocities.get_mut(&target).unwrap().1 -= ATTRACTION * dy;
        }
        
        // Update positions
        for node in graph.node_indices() {
            let vel = velocities.get_mut(&node).unwrap();
            vel.0 *= DAMPING;
            vel.1 *= DAMPING;
            
            let pos = positions.get_mut(&node).unwrap();
            pos.0 += vel.0;
            pos.1 += vel.1;
        }
    }
    
    positions
}

fn detect_communities(graph: &Graph<Note, Link>) -> Vec<Cluster> {
    // Implement Louvain community detection
    // Returns clusters of related notes
    vec![]
}

fn calculate_graph_stats(graph: &Graph<Note, Link>) -> GraphStats {
    GraphStats {
        node_count: graph.node_count(),
        edge_count: graph.edge_count(),
        average_degree: (2.0 * graph.edge_count() as f32) / graph.node_count() as f32,
        clustering_coefficient: calculate_clustering_coefficient(graph),
        connected_components: count_connected_components(graph),
    }
}
```

### gRPC Service Definition

```proto
// protos/knowledge_graph.proto
syntax = "proto3";
package altair.knowledge.graph;

service KnowledgeGraphService {
  rpc GetGraph(GetGraphRequest) returns (GraphData);
  rpc GetSubgraph(GetSubgraphRequest) returns (GraphData);
  rpc FindPath(FindPathRequest) returns (PathResult);
  rpc GetNodeNeighbors(GetNeighborsRequest) returns (NeighborList);
  rpc CalculateLayout(LayoutRequest) returns (LayoutResult);
  rpc DetectCommunities(CommunityRequest) returns (CommunityList);
  rpc ExportGraph(ExportRequest) returns (ExportResult);
}

message GraphData {
  repeated GraphNode nodes = 1;
  repeated GraphEdge edges = 2;
  repeated Cluster clusters = 3;
  GraphStats stats = 4;
}

message GraphNode {
  string id = 1;
  string label = 2;
  float x = 3;
  float y = 4;
  float size = 5;
  string color = 6;
  string cluster_id = 7;
  NodeMetadata metadata = 8;
}

message GraphEdge {
  string source = 1;
  string target = 2;
  float weight = 3;
  EdgeType type = 4;
  string label = 5;
}

message NodeMetadata {
  int32 connection_count = 1;
  repeated string tags = 2;
  int64 created_at = 3;
  int64 updated_at = 4;
  int32 word_count = 5;
}

message GraphStats {
  int32 node_count = 1;
  int32 edge_count = 2;
  float average_degree = 3;
  float clustering_coefficient = 4;
  int32 connected_components = 5;
  repeated string most_connected = 6;
}

enum LayoutAlgorithm {
  FORCE_DIRECTED = 0;
  HIERARCHICAL = 1;
  CIRCULAR = 2;
  GRID = 3;
  RANDOM = 4;
}

enum EdgeType {
  EXPLICIT_LINK = 0;
  TAG_SIMILARITY = 1;
  TEMPORAL_PROXIMITY = 2;
  SEMANTIC_SIMILARITY = 3;
}
```

### Canvas Rendering Implementation

```dart
// lib/features/knowledge_graph/presentation/widgets/graph_canvas.dart
import 'package:flutter/material.dart';
import 'dart:ui' as ui;

class GraphCanvas extends StatefulWidget {
  final GraphData graphData;
  
  @override
  _GraphCanvasState createState() => _GraphCanvasState();
}

class _GraphCanvasState extends State<GraphCanvas> with TickerProviderStateMixin {
  late AnimationController _animationController;
  final TransformationController _transformController = TransformationController();
  
  @override
  Widget build(BuildContext context) {
    return InteractiveViewer(
      transformationController: _transformController,
      minScale: 0.1,
      maxScale: 5.0,
      boundaryMargin: EdgeInsets.all(double.infinity),
      child: CustomPaint(
        size: Size.infinite,
        painter: GraphPainter(
          nodes: widget.graphData.nodes,
          edges: widget.graphData.edges,
          selectedNode: ref.watch(selectedNodeProvider),
          animationValue: _animationController.value,
        ),
      ),
    );
  }
}

class GraphPainter extends CustomPainter {
  final List<GraphNode> nodes;
  final List<GraphEdge> edges;
  final String? selectedNode;
  final double animationValue;
  
  @override
  void paint(Canvas canvas, Size size) {
    // Draw edges
    final edgePaint = Paint()
      ..color = Colors.grey.withOpacity(0.5)
      ..strokeWidth = 1.0
      ..style = PaintingStyle.stroke;
    
    for (final edge in edges) {
      final source = nodes.firstWhere((n) => n.id == edge.source);
      final target = nodes.firstWhere((n) => n.id == edge.target);
      
      canvas.drawLine(
        Offset(source.x, source.y),
        Offset(target.x, target.y),
        edgePaint,
      );
    }
    
    // Draw nodes
    for (final node in nodes) {
      final isSelected = node.id == selectedNode;
      final nodeRadius = node.size * (isSelected ? 1.5 : 1.0);
      
      // Node shadow
      final shadowPaint = Paint()
        ..color = Colors.black.withOpacity(0.2)
        ..maskFilter = MaskFilter.blur(BlurStyle.normal, 4);
      
      canvas.drawCircle(
        Offset(node.x + 2, node.y + 2),
        nodeRadius,
        shadowPaint,
      );
      
      // Node fill
      final nodePaint = Paint()
        ..color = Color(int.parse(node.color.replaceFirst('#', '0xff')))
        ..style = PaintingStyle.fill;
      
      canvas.drawCircle(
        Offset(node.x, node.y),
        nodeRadius,
        nodePaint,
      );
      
      // Node border
      final borderPaint = Paint()
        ..color = isSelected ? Colors.green : Colors.black
        ..strokeWidth = isSelected ? 3.0 : 2.0
        ..style = PaintingStyle.stroke;
      
      canvas.drawCircle(
        Offset(node.x, node.y),
        nodeRadius,
        borderPaint,
      );
      
      // Node label
      final textPainter = TextPainter(
        text: TextSpan(
          text: node.label,
          style: TextStyle(
            color: Colors.black,
            fontSize: 12,
            fontWeight: isSelected ? FontWeight.bold : FontWeight.normal,
          ),
        ),
        textDirection: TextDirection.ltr,
      );
      
      textPainter.layout();
      textPainter.paint(
        canvas,
        Offset(
          node.x - textPainter.width / 2,
          node.y + nodeRadius + 5,
        ),
      );
    }
  }
  
  @override
  bool shouldRepaint(covariant CustomPainter oldDelegate) => true;
}
```

### SurrealDB Schema

```sql
-- Graph cache table
DEFINE TABLE graph_cache SCHEMAFULL;
DEFINE FIELD graph_data ON graph_cache TYPE object;
DEFINE FIELD layout ON graph_cache TYPE string;
DEFINE FIELD generated_at ON graph_cache TYPE datetime DEFAULT time::now();
DEFINE FIELD node_count ON graph_cache TYPE int;
DEFINE FIELD edge_count ON graph_cache TYPE int;

-- Graph layouts table
DEFINE TABLE graph_layouts SCHEMAFULL;
DEFINE FIELD name ON graph_layouts TYPE string;
DEFINE FIELD positions ON graph_layouts TYPE object;
DEFINE FIELD algorithm ON graph_layouts TYPE string;
DEFINE FIELD created_at ON graph_layouts TYPE datetime DEFAULT time::now();

-- Function to get graph data
DEFINE FUNCTION fn::get_graph_data() {
  LET $nodes = (
    SELECT id, title as label, 
           count(<-links) + count(->links) as connection_count,
           tags
    FROM notes
  );
  
  LET $edges = (
    SELECT in as source, out as target, 1 as weight
    FROM links
  );
  
  RETURN {
    nodes: $nodes,
    edges: $edges,
    stats: fn::calculate_graph_stats($nodes, $edges)
  };
};

-- Function to find shortest path
DEFINE FUNCTION fn::find_shortest_path($from: string, $to: string) {
  -- Use SurrealDB's graph traversal
  RETURN SELECT * FROM $from->links*->$to LIMIT 1;
};
```

## Testing Requirements

### Unit Tests

```dart
// test/features/knowledge_graph/domain/algorithms/force_directed_test.dart
void main() {
  group('ForceDirectedLayout', () {
    test('calculates stable layout', () {
      final nodes = generateTestNodes(50);
      final edges = generateTestEdges(nodes, 100);
      final layout = ForceDirectedLayout();
      
      final positions = layout.calculate(nodes, edges);
      
      expect(positions.length, equals(nodes.length));
      // Verify no overlapping nodes
      for (int i = 0; i < nodes.length; i++) {
        for (int j = i + 1; j < nodes.length; j++) {
          final dist = distance(positions[i], positions[j]);
          expect(dist, greaterThan(nodes[i].size + nodes[j].size));
        }
      }
    });
  });
}
```

### Widget Tests

```dart
// test/features/knowledge_graph/presentation/widgets/graph_canvas_test.dart
void main() {
  testWidgets('Graph canvas renders nodes and edges', (tester) async {
    final graphData = GraphData(
      nodes: [/* test nodes */],
      edges: [/* test edges */],
    );
    
    await tester.pumpWidget(GraphCanvas(graphData: graphData));
    
    expect(find.byType(CustomPaint), findsOneWidget);
  });
}
```

### Integration Tests

```dart
// integration_test/graph_navigation_test.dart
void main() {
  testWidgets('Graph interaction and navigation', (tester) async {
    await tester.pumpWidget(MyApp());
    
    // Open graph view
    await tester.tap(find.byIcon(Icons.hub));
    await tester.pumpAndSettle();
    
    // Verify graph loaded
    expect(find.byType(GraphCanvas), findsOneWidget);
    
    // Click on a node
    await tester.tapAt(Offset(400, 300)); // Assuming node position
    await tester.pumpAndSettle();
    
    // Verify node selected
    expect(find.byType(NodeTooltip), findsOneWidget);
  });
}
```

### Accessibility Tests

- [ ] Graph structure announced to screen reader
- [ ] Keyboard navigation through nodes
- [ ] High contrast mode works
- [ ] Reduced motion respected

## Definition of Done

- [ ] Graph renders with 1000+ nodes smoothly
- [ ] Force-directed layout works correctly
- [ ] Pan/zoom navigation smooth at 60fps
- [ ] Node interactions responsive
- [ ] Filters apply instantly
- [ ] Path finding works
- [ ] Export to image functional
- [ ] All tests passing (>80% coverage)
- [ ] Accessibility audit complete
- [ ] Performance benchmarks met
- [ ] Documentation updated
- [ ] Code review approved
