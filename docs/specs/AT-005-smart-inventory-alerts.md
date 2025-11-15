# Feature AT-005: Smart Inventory Alerts

## What it does

Monitors inventory levels and sends intelligent alerts when items are low, depleted, or require reordering. Tracks consumption patterns, predicts future needs, and automatically generates shopping lists. Supports customizable thresholds per item and category-based alerting for makers and hardware enthusiasts.

## User Journey

**Scenario 1: Low Stock Alert**
```
GIVEN user has set minimum threshold of 10 for resistors
WHEN resistor quantity drops to 9 after project usage
THEN system sends notification "Resistors low: 9 remaining (min 10)"
AND adds resistors to shopping list
AND suggests typical reorder quantity based on usage history
```

**Scenario 2: Consumption Tracking**
```
GIVEN user tracks solder wire consumption
WHEN user marks 50g of solder used in project
THEN system updates remaining quantity
AND calculates consumption rate (g per week)
AND predicts "Solder will run out in ~3 weeks"
```

**Scenario 3: Batch Alerts**
```
GIVEN user has 15 items below threshold
WHEN user opens Tracking app
THEN dashboard shows "15 items need restocking" banner
AND provides quick action to review all low items
AND generates consolidated shopping list grouped by distributor
```

## Functional Requirements

### FR-1: Threshold Management
- Set minimum and maximum quantity thresholds per item
- Support different threshold types:
  - Absolute quantity (e.g., "minimum 10 units")
  - Percentage of max capacity (e.g., "alert at 20% remaining")
  - Time-based (e.g., "alert 2 weeks before depletion")
- Default thresholds by category (resistors default min=50, ICs default min=5)
- Bulk threshold setting for similar items
- Threshold presets for common component types

### FR-2: Alert Delivery
- Multiple alert channels:
  - Desktop notifications (SHARED-003)
  - Mobile push notifications
  - Email digests (daily/weekly summary)
  - In-app notification center
- Alert priority levels:
  - Critical: Out of stock
  - High: Below minimum threshold
  - Medium: Approaching threshold (within 20%)
  - Low: Suggested reorder based on usage patterns
- Customizable notification preferences per priority level
- Snooze alerts (1 day, 3 days, 1 week, custom)
- Batch notifications: Group similar alerts to reduce notification fatigue

### FR-3: Consumption Tracking
- Manual consumption entry (used X units on date Y)
- Automatic consumption via project integration (BoM tracking)
- Consumption history with charts (usage over time)
- Calculate average consumption rate (units per week/month)
- Identify consumption trends (increasing, decreasing, seasonal)
- Track consumption by project, category, or time period

### FR-4: Predictive Analytics
- Predict when item will run out based on consumption rate
- Suggest reorder timing with lead time consideration
- Recommend reorder quantity based on:
  - Historical usage patterns
  - Current project pipeline (from Guidance)
  - Storage capacity constraints
  - Bulk purchase discounts (future)
- Identify slow-moving inventory (unused for >6 months)
- Detect unusual consumption spikes (alert for potential errors)

### FR-5: Shopping List Generation
- Automatically add low-stock items to shopping list
- Group items by distributor/vendor for efficient ordering
- Calculate total cost estimate
- Prioritize items by criticality (blocking active projects)
- Shareable shopping lists (export as CSV, PDF, or share link)
- Mark items as ordered/received with purchase tracking

### FR-6: Category-Level Alerting
- Alert when entire category is low (e.g., "Most resistors are low")
- Inventory health score per category (0-100 scale)
- Visual dashboard showing category status
- Category-specific reorder recommendations

## UI/UX Requirements

### Components

**Flutter Widgets to Create:**
- `AlertDashboardCard` - Summary of all active alerts
- `LowStockBadge` - Visual indicator on item cards
- `ThresholdSettingsWidget` - Configure item thresholds
- `ConsumptionChartWidget` - Visual consumption trends
- `ShoppingListWidget` - Interactive shopping list
- `NotificationPreferencesPanel` - Alert configuration
- `InventoryHealthCard` - Category health overview
- `PredictiveInsightsPanel` - Future stock predictions

**Design System Components:**
- `AlertBanner` (critical, warning, info)
- `Badge` (count indicator)
- `ProgressBar` (stock level visualization)
- `LineChart` (consumption trends)
- `PieChart` (inventory distribution)
- `Checkbox` for shopping list
- `Switch` for notification toggles

### Visual Design

**Layout:**
- Desktop: Alert dashboard in left sidebar (250px), main content area for details
- Tablet: Collapsible alert panel, swipe from left edge to reveal
- Mobile: Bottom navigation with badge count, full-screen alert view
- Grid spacing: 12px between alert items, 20px between sections

**Colors:**
- Critical alerts: Red (#DC2626) with pulsing animation
- High priority: Orange (#F97316)
- Medium priority: Yellow (#F59E0B)
- Low priority: Blue (#3B82F6)
- Success (resolved): Green (#10B981)
- Background: White (#FFFFFF) cards, Light gray (#F9FAFB) dashboard
- Text: Dark gray (#111827) primary, Medium gray (#6B7280) secondary
- Borders: 2px solid black (neo-brutalist)

**Typography:**
- Alert titles: Inter Bold, 16px
- Alert body: Inter Regular, 14px
- Timestamps: Inter Regular, 12px, gray
- Numbers (quantities): JetBrains Mono Bold, 18px
- Line height: 1.5

**Iconography:**
- Alert: `exclamationmark_triangle_fill` (24px)
- Low stock: `arrow_down_circle` (20px)
- Shopping cart: `cart_fill` (24px)
- Consumption: `chart_line_uptrend_xyaxis` (20px)
- Threshold: `gauge` (20px)
- Notification: `bell_fill` (20px)

**Borders/Shadows:**
- Alert cards: 2px solid black, 3px offset shadow (priority-colored)
- Critical alerts: 4px shadow for emphasis
- Interactive elements: Translate (-2px, -2px) on hover

### User Interactions

**Input Methods:**
- **Click:** Dismiss alert, mark as resolved, view details
- **Double-click:** Quick snooze (default 1 day)
- **Right-click:** Context menu (snooze, ignore, add to shopping list)
- **Drag-and-drop:** Drag alert to shopping list area

**Keyboard Shortcuts:**
- `Ctrl+Shift+A` - View all alerts
- `Ctrl+Shift+S` - Open shopping list
- `Space` - Mark alert as resolved
- `S` - Snooze selected alert
- `Delete` - Dismiss alert
- `Escape` - Close alert detail view

**Gestures (Mobile):**
- Swipe right: Mark as resolved
- Swipe left: Snooze
- Long-press: Multi-select alerts
- Pull to refresh: Check for new alerts

**Feedback:**
- Alert appears: Slide-in animation with sound (configurable)
- Resolved: Fade-out with success checkmark
- Snoozed: Dim and slide away
- Error: Shake animation + error message
- All animations: <300ms duration

### State Management

**Local State:**
- Alert filter selections (priority, category)
- Shopping list item checkboxes
- Form inputs (threshold settings)
- Chart time range selection

**Global State (Riverpod Providers):**
```dart
// Core state
final alertsProvider = StateNotifierProvider<AlertsNotifier, List<InventoryAlert>>;
final thresholdsProvider = StateNotifierProvider<ThresholdsNotifier, Map<String, Threshold>>;
final consumptionHistoryProvider = StateNotifierProvider<ConsumptionHistoryNotifier, List<ConsumptionEntry>>;
final shoppingListProvider = StateNotifierProvider<ShoppingListNotifier, ShoppingList>;

// Filters and settings
final alertFiltersProvider = StateProvider<AlertFilters>;
final notificationPreferencesProvider = StateProvider<NotificationPreferences>;

// Computed providers
final activeAlertsProvider = Provider<List<InventoryAlert>>;  // Un-snoozed, unresolved
final criticalAlertsCountProvider = Provider<int>;
final shoppingListTotalProvider = Provider<double>;  // Total estimated cost
final inventoryHealthProvider = Provider<Map<String, double>>;  // Category health scores

// Async providers
final consumptionTrendsProvider = FutureProvider.family<ConsumptionTrend, String>;  // Item ID
final predictedDepletionProvider = FutureProvider.family<PredictionResult, String>;
```

**Persistence:**
- Alert snooze state persists across app restarts
- Notification preferences saved to user settings
- Shopping list auto-saves every 10 seconds
- Consumption history synced to backend immediately
- Threshold settings cached locally with background sync

### Responsive Behavior

**Desktop (>1200px):**
- Left sidebar (250px) with alert list, always visible
- Main content area with charts and detailed views
- Hover tooltips on all data points
- Side-by-side comparison views

**Tablet (768-1199px):**
- Collapsible alert sidebar, swipe from left to reveal
- Tab navigation for Alerts | Shopping List | Analytics
- Touch-optimized controls (larger tap targets)

**Mobile (<768px):**
- Bottom tab navigation with badge counts
- Full-screen alert view
- Swipe gestures for quick actions
- Floating action button (FAB) for shopping list access

**Breakpoint Strategy:**
- Mobile-first design with progressive enhancement
- Shared business logic across all breakpoints
- Responsive charts that adapt to container size

### Accessibility Requirements

**Screen Reader:**
- Alert announcements: "Critical alert: Resistors low, 9 remaining, minimum 10"
- Shopping list items: "Item: Solder wire, quantity 2, estimated cost $15"
- Live region for new alerts (polite mode to avoid interruption)
- Progress bars have aria-valuenow, aria-valuemin, aria-valuemax

**Keyboard Navigation:**
- Tab through all alerts in priority order (critical first)
- Arrow keys to navigate within shopping list
- Enter to activate/resolve alerts
- Focus returns to trigger element after modal close

**Color Contrast:**
- All alert colors meet WCAG AA standards with text contrast >4.5:1
- Priority levels indicated by both color and icon
- High contrast mode increases border thickness to 3px

**Motion:**
- Respect prefers-reduced-motion for slide/fade animations
- Disable pulsing animations if motion reduced
- Critical alerts use vibration pattern on mobile (if supported)

**Font Sizing:**
- Minimum 14px font size
- Scalable text up to 200% without layout breaking
- Numbers use tabular figures for alignment

### ADHD-Specific UI Requirements

**Cognitive Load Reduction:**
- Show only critical/high priority alerts by default
- "Show less critical" toggle to reveal medium/low alerts
- Limit visible alerts to 10, "Load more" button for others
- Group similar alerts (e.g., "5 resistor types are low")

**Focus Management:**
- Autofocus on most critical alert when alert dashboard opens
- Critical alerts have subtle pulsing border (2s interval)
- "Focus Mode": Show only blocking alerts (items needed for active projects)

**Forgiveness Features:**
- Snooze alerts without guilt: "I'll handle this later"
- Dismiss alerts: "I know, I'm working on it"
- Restore dismissed alerts: "Oops, I need this after all"
- No shame language: "Low stock" not "You're running out!"

**Visual Hierarchy:**
- Alert count badge in navigation (large, bold)
- Critical alerts at top with red accent
- Resolved alerts fade to background (low opacity)
- Primary action (Add to shopping list) always top-right

**Immediate Feedback:**
- Alert resolution: Instant success animation (<100ms)
- Shopping list add: Item slides into list with haptic feedback
- Threshold update: Visual confirmation + toast
- Optimistic UI: Assume success, rollback on error

**Time Blindness Support:**
- "Running out in X days" with visual countdown
- Color-coded urgency: Red (<7 days), Orange (7-14 days), Yellow (14-30 days)
- Predict when to order with lead time buffer
- Calendar integration showing when items will deplete

## Non-Functional Requirements

### Performance Targets

- Alert dashboard load: <150ms for 100 alerts
- Consumption chart render: <200ms for 1 year of data
- Shopping list update: <50ms per item
- Notification delivery: <3s from threshold trigger
- Prediction calculation: <500ms per item
- Batch alert processing: <2s for 100+ items
- Animation frame rate: 60fps for all transitions

### Technical Constraints

**Flutter Dependencies:**
```yaml
dependencies:
  flutter_riverpod: ^2.4.0
  fl_chart: ^0.65.0  # Charts
  flutter_local_notifications: ^16.3.0  # Local notifications
  workmanager: ^0.5.1  # Background tasks
  collection: ^1.18.0  # Data manipulation
```

**Rust Backend Dependencies:**
```toml
[dependencies]
tokio-cron-scheduler = "0.10"  # Scheduled alert checks
lettre = "0.11"  # Email notifications (optional)
```

### Security Requirements

- Notification content does not expose sensitive inventory details
- Alert API endpoints require authentication
- Rate limiting on alert queries (100 requests/minute per user)
- Shopping list data encrypted at rest

## Implementation Details

### Code Structure

```
lib/
├── features/
│   └── alerts/
│       ├── presentation/
│       │   ├── screens/
│       │   │   ├── alert_dashboard_screen.dart
│       │   │   ├── shopping_list_screen.dart
│       │   │   └── threshold_settings_screen.dart
│       │   ├── widgets/
│       │   │   ├── alert_dashboard_card.dart
│       │   │   ├── low_stock_badge.dart
│       │   │   ├── threshold_settings_widget.dart
│       │   │   ├── consumption_chart_widget.dart
│       │   │   ├── shopping_list_widget.dart
│       │   │   ├── notification_preferences_panel.dart
│       │   │   ├── inventory_health_card.dart
│       │   │   └── predictive_insights_panel.dart
│       │   └── providers/
│       │       ├── alerts_provider.dart
│       │       ├── thresholds_provider.dart
│       │       ├── consumption_history_provider.dart
│       │       └── shopping_list_provider.dart
│       ├── domain/
│       │   ├── models/
│       │   │   ├── inventory_alert.dart
│       │   │   ├── threshold.dart
│       │   │   ├── consumption_entry.dart
│       │   │   └── shopping_list.dart
│       │   ├── repositories/
│       │   │   ├── alert_repository.dart
│       │   │   └── consumption_repository.dart
│       │   └── use_cases/
│       │       ├── check_thresholds.dart
│       │       ├── track_consumption.dart
│       │       ├── predict_depletion.dart
│       │       └── generate_shopping_list.dart
│       └── data/
│           ├── repositories/
│           │   ├── alert_repository_impl.dart
│           │   └── consumption_repository_impl.dart
│           └── data_sources/
│               ├── alert_local_data_source.dart
│               └── alert_remote_data_source.dart
```

### Key Files to Create

1. `alert_dashboard_screen.dart` - Main alert view
2. `alerts_provider.dart` - Riverpod state management
3. `check_thresholds.dart` - Use case for threshold checking
4. `alert_service.rs` - Rust backend service
5. `alert_service.proto` - gRPC definitions

### gRPC Service Definition

```protobuf
syntax = "proto3";

package altair.tracking.alerts;

import "google/protobuf/timestamp.proto";

service AlertService {
  rpc CheckThresholds(CheckThresholdsRequest) returns (CheckThresholdsResponse);
  rpc GetActiveAlerts(GetActiveAlertsRequest) returns (GetActiveAlertsResponse);
  rpc ResolveAlert(ResolveAlertRequest) returns (AlertResponse);
  rpc SnoozeAlert(SnoozeAlertRequest) returns (AlertResponse);
  rpc UpdateThreshold(UpdateThresholdRequest) returns (ThresholdResponse);
  rpc TrackConsumption(TrackConsumptionRequest) returns (ConsumptionResponse);
  rpc GetConsumptionHistory(GetConsumptionHistoryRequest) returns (GetConsumptionHistoryResponse);
  rpc PredictDepletion(PredictDepletionRequest) returns (PredictionResponse);
  rpc GenerateShoppingList(GenerateShoppingListRequest) returns (ShoppingListResponse);
}

message InventoryAlert {
  string id = 1;
  string item_id = 2;
  AlertType type = 3;
  AlertPriority priority = 4;
  string message = 5;
  int32 current_quantity = 6;
  int32 threshold_quantity = 7;
  google.protobuf.Timestamp created_at = 8;
  google.protobuf.Timestamp snoozed_until = 9;
  bool is_resolved = 10;
}

enum AlertType {
  ALERT_TYPE_UNSPECIFIED = 0;
  ALERT_TYPE_LOW_STOCK = 1;
  ALERT_TYPE_OUT_OF_STOCK = 2;
  ALERT_TYPE_APPROACHING_THRESHOLD = 3;
  ALERT_TYPE_PREDICTED_DEPLETION = 4;
  ALERT_TYPE_SLOW_MOVING = 5;
  ALERT_TYPE_CONSUMPTION_SPIKE = 6;
}

enum AlertPriority {
  ALERT_PRIORITY_UNSPECIFIED = 0;
  ALERT_PRIORITY_CRITICAL = 1;
  ALERT_PRIORITY_HIGH = 2;
  ALERT_PRIORITY_MEDIUM = 3;
  ALERT_PRIORITY_LOW = 4;
}

message Threshold {
  string item_id = 1;
  ThresholdType type = 2;
  int32 min_quantity = 3;
  int32 max_quantity = 4;
  double percentage = 5;  // For percentage-based thresholds
  int32 lead_time_days = 6;
  google.protobuf.Timestamp updated_at = 7;
}

enum ThresholdType {
  THRESHOLD_TYPE_UNSPECIFIED = 0;
  THRESHOLD_TYPE_ABSOLUTE = 1;
  THRESHOLD_TYPE_PERCENTAGE = 2;
  THRESHOLD_TYPE_TIME_BASED = 3;
}

message ConsumptionEntry {
  string id = 1;
  string item_id = 2;
  int32 quantity_used = 3;
  google.protobuf.Timestamp consumption_date = 4;
  string project_id = 5;  // Optional, from Guidance
  string notes = 6;
  google.protobuf.Timestamp created_at = 7;
}

message ConsumptionTrend {
  string item_id = 1;
  double average_daily_consumption = 2;
  double average_weekly_consumption = 3;
  double average_monthly_consumption = 4;
  TrendDirection trend = 5;
  repeated DataPoint history = 6;
}

enum TrendDirection {
  TREND_DIRECTION_UNSPECIFIED = 0;
  TREND_DIRECTION_INCREASING = 1;
  TREND_DIRECTION_DECREASING = 2;
  TREND_DIRECTION_STABLE = 3;
}

message DataPoint {
  google.protobuf.Timestamp date = 1;
  double value = 2;
}

message PredictionResult {
  string item_id = 1;
  google.protobuf.Timestamp predicted_depletion_date = 2;
  int32 days_until_depletion = 3;
  google.protobuf.Timestamp recommended_reorder_date = 4;
  int32 recommended_reorder_quantity = 5;
  double confidence_score = 6;  // 0.0 to 1.0
}

message ShoppingList {
  string id = 1;
  repeated ShoppingListItem items = 2;
  double total_estimated_cost = 3;
  google.protobuf.Timestamp created_at = 4;
  google.protobuf.Timestamp updated_at = 5;
}

message ShoppingListItem {
  string item_id = 1;
  string item_name = 2;
  int32 quantity_needed = 3;
  double estimated_unit_cost = 4;
  string preferred_vendor = 5;
  bool is_critical = 6;  // Blocks active projects
  bool is_ordered = 7;
}

message CheckThresholdsRequest {}

message CheckThresholdsResponse {
  repeated InventoryAlert new_alerts = 1;
  int32 items_checked = 2;
}

message GetActiveAlertsRequest {
  repeated AlertPriority priorities = 1;  // Filter
  bool include_snoozed = 2;
}

message GetActiveAlertsResponse {
  repeated InventoryAlert alerts = 1;
  int32 total_count = 2;
}

message ResolveAlertRequest {
  string alert_id = 1;
}

message SnoozeAlertRequest {
  string alert_id = 1;
  google.protobuf.Timestamp snooze_until = 2;
}

message AlertResponse {
  InventoryAlert alert = 1;
}

message UpdateThresholdRequest {
  Threshold threshold = 1;
}

message ThresholdResponse {
  Threshold threshold = 1;
}

message TrackConsumptionRequest {
  ConsumptionEntry entry = 1;
}

message ConsumptionResponse {
  ConsumptionEntry entry = 1;
}

message GetConsumptionHistoryRequest {
  string item_id = 1;
  google.protobuf.Timestamp start_date = 2;
  google.protobuf.Timestamp end_date = 3;
}

message GetConsumptionHistoryResponse {
  repeated ConsumptionEntry entries = 1;
  ConsumptionTrend trend = 2;
}

message PredictDepletionRequest {
  string item_id = 1;
}

message PredictionResponse {
  PredictionResult prediction = 1;
}

message GenerateShoppingListRequest {
  bool include_predicted = 1;  // Include items predicted to deplete soon
}

message ShoppingListResponse {
  ShoppingList shopping_list = 1;
}
```

### SurrealDB Schema

```sql
-- Inventory alerts table
DEFINE TABLE inventory_alerts SCHEMAFULL;
DEFINE FIELD item_id ON inventory_alerts TYPE record<items>;
DEFINE FIELD type ON inventory_alerts TYPE string
  ASSERT $value IN ['low_stock', 'out_of_stock', 'approaching_threshold', 'predicted_depletion', 'slow_moving', 'consumption_spike'];
DEFINE FIELD priority ON inventory_alerts TYPE string
  ASSERT $value IN ['critical', 'high', 'medium', 'low'];
DEFINE FIELD message ON inventory_alerts TYPE string;
DEFINE FIELD current_quantity ON inventory_alerts TYPE int;
DEFINE FIELD threshold_quantity ON inventory_alerts TYPE int;
DEFINE FIELD created_at ON inventory_alerts TYPE datetime DEFAULT time::now();
DEFINE FIELD snoozed_until ON inventory_alerts TYPE option<datetime>;
DEFINE FIELD is_resolved ON inventory_alerts TYPE bool DEFAULT false;

DEFINE INDEX alert_item_idx ON inventory_alerts FIELDS item_id;
DEFINE INDEX alert_priority_idx ON inventory_alerts FIELDS priority;
DEFINE INDEX alert_resolved_idx ON inventory_alerts FIELDS is_resolved;

-- Thresholds table
DEFINE TABLE thresholds SCHEMAFULL;
DEFINE FIELD item_id ON thresholds TYPE record<items>;
DEFINE FIELD type ON thresholds TYPE string
  ASSERT $value IN ['absolute', 'percentage', 'time_based'];
DEFINE FIELD min_quantity ON thresholds TYPE int;
DEFINE FIELD max_quantity ON thresholds TYPE option<int>;
DEFINE FIELD percentage ON thresholds TYPE option<float>;
DEFINE FIELD lead_time_days ON thresholds TYPE int DEFAULT 7;
DEFINE FIELD updated_at ON thresholds TYPE datetime DEFAULT time::now();

DEFINE INDEX threshold_item_idx ON thresholds FIELDS item_id;

-- Consumption history table
DEFINE TABLE consumption_history SCHEMAFULL;
DEFINE FIELD item_id ON consumption_history TYPE record<items>;
DEFINE FIELD quantity_used ON consumption_history TYPE int;
DEFINE FIELD consumption_date ON consumption_history TYPE datetime;
DEFINE FIELD project_id ON consumption_history TYPE option<string>;
DEFINE FIELD notes ON consumption_history TYPE string;
DEFINE FIELD created_at ON consumption_history TYPE datetime DEFAULT time::now();

DEFINE INDEX consumption_item_idx ON consumption_history FIELDS item_id;
DEFINE INDEX consumption_date_idx ON consumption_history FIELDS consumption_date;

-- Shopping lists table
DEFINE TABLE shopping_lists SCHEMAFULL;
DEFINE FIELD items ON shopping_lists TYPE array<object>;
DEFINE FIELD total_estimated_cost ON shopping_lists TYPE decimal DEFAULT 0.0;
DEFINE FIELD created_at ON shopping_lists TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON shopping_lists TYPE datetime DEFAULT time::now();
```

### Rust Backend Implementation

```rust
// src/services/alert_service.rs
use crate::models::{InventoryAlert, Threshold, ConsumptionEntry, PredictionResult};
use crate::repositories::AlertRepository;
use chrono::{DateTime, Utc, Duration};

pub struct AlertService {
    repository: AlertRepository,
}

impl AlertService {
    pub fn new(repository: AlertRepository) -> Self {
        Self { repository }
    }
    
    /// Check all items against their thresholds and generate alerts
    pub async fn check_thresholds(&self) -> Result<Vec<InventoryAlert>, ServiceError> {
        let items = self.repository.get_all_items().await?;
        let mut new_alerts = Vec::new();
        
        for item in items {
            if let Some(threshold) = self.repository.get_threshold(&item.id).await? {
                if let Some(alert) = self.evaluate_threshold(&item, &threshold).await? {
                    new_alerts.push(alert);
                }
            }
        }
        
        // Save alerts to database
        for alert in &new_alerts {
            self.repository.create_alert(alert.clone()).await?;
        }
        
        Ok(new_alerts)
    }
    
    /// Evaluate if an item violates its threshold
    async fn evaluate_threshold(
        &self,
        item: &Item,
        threshold: &Threshold,
    ) -> Result<Option<InventoryAlert>, ServiceError> {
        let current_qty = item.quantity;
        
        let (alert_type, priority) = match threshold.type_.as_str() {
            "absolute" => {
                if current_qty == 0 {
                    Some(("out_of_stock", "critical"))
                } else if current_qty < threshold.min_quantity {
                    Some(("low_stock", "high"))
                } else if current_qty < (threshold.min_quantity as f64 * 1.2) as i32 {
                    Some(("approaching_threshold", "medium"))
                } else {
                    None
                }
            }
            "percentage" => {
                if let Some(max_qty) = threshold.max_quantity {
                    let percentage = (current_qty as f64 / max_qty as f64) * 100.0;
                    if percentage < threshold.percentage.unwrap_or(20.0) {
                        Some(("low_stock", "high"))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        };
        
        if let Some((alert_type, priority)) = alert_type {
            let message = format!(
                "{} is {}: {} remaining (min {})",
                item.name, alert_type.replace('_', " "), current_qty, threshold.min_quantity
            );
            
            Ok(Some(InventoryAlert {
                id: uuid::Uuid::new_v4().to_string(),
                item_id: item.id.clone(),
                type_: alert_type.to_string(),
                priority: priority.to_string(),
                message,
                current_quantity: current_qty,
                threshold_quantity: threshold.min_quantity,
                created_at: Utc::now(),
                snoozed_until: None,
                is_resolved: false,
            }))
        } else {
            Ok(None)
        }
    }
    
    /// Track consumption and update item quantity
    pub async fn track_consumption(
        &self,
        entry: ConsumptionEntry,
    ) -> Result<ConsumptionEntry, ServiceError> {
        // Save consumption entry
        let mut entry = entry;
        entry.id = uuid::Uuid::new_v4().to_string();
        entry.created_at = Utc::now();
        self.repository.create_consumption_entry(&entry).await?;
        
        // Update item quantity
        self.repository.decrement_item_quantity(&entry.item_id, entry.quantity_used).await?;
        
        // Check if this triggers any alerts
        self.check_thresholds().await?;
        
        Ok(entry)
    }
    
    /// Predict when an item will run out based on consumption history
    pub async fn predict_depletion(
        &self,
        item_id: &str,
    ) -> Result<PredictionResult, ServiceError> {
        let item = self.repository.get_item(item_id).await?;
        let history = self.repository.get_consumption_history(item_id, None, None).await?;
        
        if history.len() < 3 {
            return Err(ServiceError::InsufficientData("Need at least 3 data points"));
        }
        
        // Calculate average daily consumption
        let total_days = (history.last().unwrap().consumption_date - history.first().unwrap().consumption_date).num_days() as f64;
        let total_consumed: i32 = history.iter().map(|e| e.quantity_used).sum();
        let avg_daily_consumption = total_consumed as f64 / total_days.max(1.0);
        
        // Predict depletion
        let days_until_depletion = if avg_daily_consumption > 0.0 {
            (item.quantity as f64 / avg_daily_consumption).ceil() as i32
        } else {
            365  // If no consumption, assume 1 year
        };
        
        let depletion_date = Utc::now() + Duration::days(days_until_depletion as i64);
        
        // Get threshold to calculate lead time
        let threshold = self.repository.get_threshold(item_id).await?.unwrap_or_default();
        let reorder_date = depletion_date - Duration::days(threshold.lead_time_days as i64);
        
        // Calculate recommended reorder quantity (enough for 30 days + buffer)
        let recommended_qty = ((avg_daily_consumption * 30.0) * 1.2).ceil() as i32;
        
        // Confidence score based on data points (more data = higher confidence)
        let confidence = (history.len() as f64 / 30.0).min(1.0);
        
        Ok(PredictionResult {
            item_id: item_id.to_string(),
            predicted_depletion_date: depletion_date,
            days_until_depletion,
            recommended_reorder_date: reorder_date,
            recommended_reorder_quantity: recommended_qty,
            confidence_score: confidence,
        })
    }
    
    /// Generate shopping list from all active alerts
    pub async fn generate_shopping_list(
        &self,
        include_predicted: bool,
    ) -> Result<ShoppingList, ServiceError> {
        let alerts = self.repository.get_active_alerts().await?;
        let mut items = Vec::new();
        
        for alert in alerts {
            let item = self.repository.get_item(&alert.item_id).await?;
            let threshold = self.repository.get_threshold(&alert.item_id).await?;
            
            let quantity_needed = if let Some(threshold) = threshold {
                threshold.min_quantity - alert.current_quantity
            } else {
                10  // Default reorder quantity
            };
            
            items.push(ShoppingListItem {
                item_id: item.id,
                item_name: item.name,
                quantity_needed,
                estimated_unit_cost: item.unit_cost.unwrap_or(0.0),
                preferred_vendor: item.preferred_vendor.unwrap_or_default(),
                is_critical: alert.priority == "critical" || alert.priority == "high",
                is_ordered: false,
            });
        }
        
        // Optionally include predicted depletions
        if include_predicted {
            // TODO: Add items that are predicted to run out soon
        }
        
        let total_cost: f64 = items.iter()
            .map(|item| item.estimated_unit_cost * item.quantity_needed as f64)
            .sum();
        
        Ok(ShoppingList {
            id: uuid::Uuid::new_v4().to_string(),
            items,
            total_estimated_cost: total_cost,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }
}
```

## Testing Requirements

### Unit Tests
- [ ] Threshold evaluation logic (absolute, percentage, time-based)
- [ ] Consumption rate calculation
- [ ] Depletion prediction algorithm
- [ ] Shopping list generation with cost calculation
- [ ] Alert priority assignment
- [ ] Snooze/resolve alert state transitions

### Widget Tests
- [ ] Alert dashboard renders all priority levels
- [ ] Low stock badge displays correct count
- [ ] Consumption chart visualizes trends
- [ ] Shopping list items check/uncheck correctly
- [ ] Notification preferences save correctly

### Integration Tests
- [ ] Threshold violation triggers alert creation
- [ ] Consumption entry updates item quantity and triggers alert check
- [ ] Predicted depletion generates alert at appropriate time
- [ ] Shopping list syncs with backend
- [ ] Notifications delivered via SHARED-003

### Accessibility Tests
- [ ] Screen reader announces new alerts
- [ ] Keyboard navigation through alert list
- [ ] Color contrast for all alert priorities
- [ ] Motion preferences respected

### Performance Tests
- [ ] Check 1000+ items against thresholds in <2s
- [ ] Load alert dashboard with 100 alerts in <150ms
- [ ] Generate shopping list from 50+ alerts in <500ms

## Definition of Done
- [ ] All functional requirements implemented
- [ ] UI matches design specifications
- [ ] All tests passing (>90% coverage)
- [ ] Accessibility audit complete (WCAG 2.1 AA)
- [ ] Performance metrics met
- [ ] Code review approved
- [ ] Documentation updated

---

## Dependencies

**Required:**
- AT-001: Item CRUD Operations
- SHARED-001: Authentication Service
- SHARED-002: Database Service (SurrealDB)
- SHARED-003: Notification Service

**Optional:**
- AT-003: BoM Intelligence (for project-based consumption tracking)
- AG-001: Quest Board (for project priority in shopping list)

---

## Future Enhancements
- Machine learning for consumption pattern detection
- Integration with distributor APIs for live pricing
- Collaborative shopping lists for shared workshops
- Voice-based consumption logging
- Computer vision for automatic stock counting
- Geofencing reminders (alert when near supplier)

---

*This specification is optimized for AI-assisted development with Cursor IDE.*
