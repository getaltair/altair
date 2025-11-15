# Feature AG-004: Gamification Engine

## What it does

Provides optional gamification layer with XP points, achievement badges, level progression, and reward shop to increase motivation and provide immediate feedback for quest completion.

## User Journey

GIVEN user has gamification enabled in settings
WHEN user completes a quest or reaches a milestone
THEN user gains XP, potentially levels up, earns achievements, and can redeem points in reward shop

## Functional Requirements

- XP point system with configurable values per quest difficulty
- 100-level progression system with exponential curve
- Achievement system with 50+ badges
- Reward shop with customizable rewards
- Streak tracking (with pause allowance)
- Leaderboard (optional, self-competition only)
- Daily/weekly challenges
- Bonus XP events
- Prestige system for level 100+
- Statistics dashboard
- Gamification on/off toggle
- Customizable difficulty settings

## UI/UX Requirements

### Components

- `XPBar` - Progress bar showing current level
- `LevelUpModal` - Celebration screen
- `AchievementToast` - Badge unlock notification
- `RewardShop` - Points redemption interface
- `StreakCounter` - Daily streak display
- `ChallengeCard` - Daily/weekly challenge widget
- `StatsDashboard` - Gamification metrics
- `AchievementGallery` - Badge collection view
- `LeaderboardView` - Personal records display
- `PrestigeIndicator` - Post-100 level display

### Visual Design

- **Layout:**
  - XP Bar: Top of screen, 40px height
  - Achievements: Toast from bottom-right
  - Reward Shop: Modal dialog, 600px width
  - Stats: Full dashboard view
- **Colors:**
  - XP Bar: Gradient `#4CAF50` to `#8BC34A`
  - Level up: Gold burst `#FFD700`
  - Achievements: Bronze/Silver/Gold/Diamond
  - Streaks: Fire gradient `#FF6B6B` to `#FFE66D`
  - Prestige: Purple shimmer `#9C27B0`
- **Typography:**
  - XP gains: 20px bold with float animation
  - Level: 24px extra-bold
  - Achievement names: 16px medium
  - Stats: 18px regular
- **Iconography:**
  - XP: Star icon (16x16)
  - Level: Shield with number
  - Achievements: Custom badge designs
  - Streaks: Fire emoji
  - Rewards: Gift box icon
- **Borders/Shadows:**
  - Level up: Particle explosion effect
  - Achievement unlock: Glow animation
  - XP bar: 2px border with inner glow

### User Interactions

- **Input Methods:**
  - Click achievements for details
  - Drag to scroll reward shop
  - Tap to claim rewards
  - Long-press for stats
- **Keyboard Shortcuts:**
  - `G`: Toggle gamification panel
  - `R`: Open reward shop
  - `A`: View achievements
  - `S`: Show stats
- **Gestures:**
  - Swipe down to dismiss notifications
  - Pull-to-refresh stats
  - Tap and hold for achievement details
- **Feedback:**
  - Sound effects for XP gain
  - Haptic on level up
  - Screen flash on achievement
  - Confetti for milestones

### State Management

- **Local State:**
  - Animation states
  - Toast queue
  - Shop scroll position
  - Selected achievement
- **Global State:**
  ```dart
  final xpProvider = StateNotifierProvider<XPNotifier, XPState>
  final levelProvider = Provider<int>((ref) => calculateLevel(ref.watch(xpProvider)))
  final achievementsProvider = StateNotifierProvider<AchievementsNotifier, List<Achievement>>
  final streakProvider = StateNotifierProvider<StreakNotifier, StreakState>
  final rewardsProvider = StateNotifierProvider<RewardsNotifier, List<Reward>>
  final gamificationEnabledProvider = StateProvider<bool>
  ```
- **Persistence:**
  - XP total saved immediately
  - Achievements cached locally
  - Streak data with timezone handling
  - Reward redemptions logged

### Responsive Behavior

- **Desktop:** Full XP bar with detailed stats
- **Tablet:** Compact XP indicator, slide-out panel
- **Mobile:** Minimal XP dot, full-screen modals
- **Breakpoint Strategy:** Progressive enhancement

### Accessibility Requirements

- **Screen Reader:**
  - Announce XP gains
  - Describe achievement unlocks
  - Read current level/streak
- **Keyboard Navigation:**
  - Tab through shop items
  - Enter to redeem rewards
- **Color Contrast:** Ensure badge visibility
- **Motion:** Option to disable animations
- **Font Sizing:** Scalable point displays

### ADHD-Specific UI Requirements

- **Cognitive Load:**
  - Optional system (off by default)
  - Simple point values (10, 25, 50, 100)
  - Clear achievement requirements
- **Focus Management:**
  - Non-blocking notifications
  - Dismiss after 3 seconds
  - Queue multiple unlocks
- **Forgiveness:**
  - Streak pauses (3 per month)
  - No XP loss ever
  - Achievements never expire
  - Retroactive XP for past work
- **Visual Hierarchy:**
  - XP gains float upward
  - Level up unmissable
  - Achievements celebrated
- **Immediate Feedback:**
  - Instant XP addition
  - <100ms achievement check
  - Real-time progress bars

## Non-Functional Requirements

### Performance Targets

- XP calculation <10ms
- Achievement check <50ms
- Level up animation 60fps
- Stats dashboard <200ms load
- Reward shop render <300ms

### Technical Constraints

- Flutter animations API
- Local SQLite for achievement data
- SurrealDB for XP/progression
- SharedPreferences for settings
- Assets folder for badge images

### Security Requirements

- Anti-cheat validation
- XP transaction integrity
- Reward redemption limits
- Achievement unlock verification

## Implementation Details

### Code Structure

```
lib/
├── features/
│   └── gamification/
│       ├── presentation/
│       │   ├── widgets/
│       │   │   ├── xp_bar.dart
│       │   │   ├── level_up_modal.dart
│       │   │   ├── achievement_toast.dart
│       │   │   ├── reward_shop.dart
│       │   │   ├── streak_counter.dart
│       │   │   └── challenge_card.dart
│       │   ├── providers/
│       │   │   ├── xp_provider.dart
│       │   │   ├── achievement_provider.dart
│       │   │   ├── streak_provider.dart
│       │   │   └── reward_provider.dart
│       │   └── screens/
│       │       ├── stats_dashboard.dart
│       │       └── achievement_gallery.dart
│       ├── domain/
│       │   ├── entities/
│       │   │   ├── xp_transaction.dart
│       │   │   ├── achievement.dart
│       │   │   ├── reward.dart
│       │   │   └── streak.dart
│       │   ├── services/
│       │   │   ├── xp_calculator.dart
│       │   │   └── achievement_checker.dart
│       │   └── repositories/
│       │       └── gamification_repository.dart
│       └── data/
│           ├── models/
│           │   └── gamification_model.dart
│           └── datasources/
│               └── gamification_datasource.dart

backend/
├── src/
│   └── services/
│       ├── gamification_service.rs
│       └── achievement_engine.rs
└── proto/
    └── gamification.proto
```

### Key Files to Create

- `xp_bar.dart` - Main XP display widget
- `xp_provider.dart` - XP state management
- `achievement_checker.dart` - Achievement logic
- `gamification_service.rs` - Backend service
- `gamification.proto` - gRPC definitions

### Dependencies

```yaml
dependencies:
  flutter_riverpod: ^2.4.0
  lottie: ^2.6.0
  confetti: ^0.7.0
  animated_text_kit: ^4.2.2
  flutter_animate: ^4.2.0
  audioplayers: ^5.0.0
  
dev_dependencies:
  integration_test:
    sdk: flutter
```

### Rust Dependencies

```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
surrealdb = "2.0"
tonic = "0.12"
serde = { version = "1.0", features = ["derive"] }
rand = "0.8"
```

## Testing Requirements

### Unit Tests

- [x] XP calculation formulas
- [x] Level progression curve
- [x] Achievement unlock conditions
- [x] Streak pause logic
- [x] Reward redemption rules

### Widget Tests

- [x] XP bar animation
- [x] Level up modal display
- [x] Achievement toast queue
- [x] Reward shop interaction
- [x] Streak counter updates

### Integration Tests

- [x] Complete quest → XP gain → Level up
- [x] Trigger achievement → Toast → Gallery
- [x] Accumulate points → Shop → Redeem
- [x] Daily login → Streak increment
- [x] Settings toggle → Hide all gamification

### Accessibility Tests

- [x] Screen reader XP announcements
- [x] Keyboard shop navigation
- [x] Motion-reduced animations
- [x] Color-blind safe badges

## Definition of Done

- [x] XP system functional
- [x] Achievements unlocking
- [x] Level progression working
- [x] Reward shop operational
- [x] Streaks tracking correctly
- [x] Challenges generating daily
- [x] Stats dashboard complete
- [x] Optional toggle working
- [x] All animations smooth
- [x] Performance targets met
- [x] Backend integration complete
