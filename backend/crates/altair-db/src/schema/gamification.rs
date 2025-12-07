//! Gamification domain types - XP, achievements, and streaks

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use super::enums::StreakType;

/// UserProgress - Player progression tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProgress {
    pub id: Option<Thing>,
    pub xp_total: i32,
    pub level: i32,
    pub title: String,
    pub owner: Thing,
    pub device_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserProgress {
    /// Calculate level from total XP (simple formula: level = sqrt(xp / 100))
    pub fn calculate_level(xp_total: i32) -> i32 {
        ((xp_total as f64 / 100.0).sqrt()).floor() as i32
    }

    /// Get XP required for next level
    pub fn xp_for_next_level(&self) -> i32 {
        let next_level = self.level + 1;
        (next_level * next_level) * 100
    }

    /// Get progress to next level (0.0 - 1.0)
    pub fn progress_to_next_level(&self) -> f32 {
        let current_level_xp = (self.level * self.level) * 100;
        let next_level_xp = self.xp_for_next_level();
        let progress = self.xp_total - current_level_xp;
        let total_needed = next_level_xp - current_level_xp;
        progress as f32 / total_needed as f32
    }

    /// Add XP and update level if needed
    pub fn add_xp(&mut self, amount: i32) {
        self.xp_total += amount;
        self.level = Self::calculate_level(self.xp_total);
        self.updated_at = Utc::now();
    }
}

/// Achievement - Unlockable milestone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: Option<Thing>,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub unlocked_at: Option<DateTime<Utc>>,
    pub owner: Thing,
    pub device_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Achievement {
    /// Check if achievement is unlocked
    pub fn is_unlocked(&self) -> bool {
        self.unlocked_at.is_some()
    }

    /// Unlock the achievement
    pub fn unlock(&mut self) {
        if self.unlocked_at.is_none() {
            self.unlocked_at = Some(Utc::now());
            self.updated_at = Utc::now();
        }
    }
}

/// Streak - Consecutive activity tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Streak {
    pub id: Option<Thing>,
    pub streak_type: StreakType,
    pub current_count: i32,
    pub longest_count: i32,
    pub last_activity: DateTime<Utc>,
    pub started_at: DateTime<Utc>,
    pub owner: Thing,
    pub device_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Streak {
    /// Increment streak counter
    pub fn increment(&mut self) {
        self.current_count += 1;
        if self.current_count > self.longest_count {
            self.longest_count = self.current_count;
        }
        self.last_activity = Utc::now();
        self.updated_at = Utc::now();
    }

    /// Reset streak (missed day)
    pub fn reset(&mut self) {
        self.current_count = 0;
        self.started_at = Utc::now();
        self.last_activity = Utc::now();
        self.updated_at = Utc::now();
    }

    /// Check if streak is still active (within 24 hours)
    pub fn is_active(&self) -> bool {
        let duration = Utc::now().signed_duration_since(self.last_activity);
        duration.num_hours() < 24
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_progress_level_calculation() {
        assert_eq!(UserProgress::calculate_level(0), 0);
        assert_eq!(UserProgress::calculate_level(100), 1);
        assert_eq!(UserProgress::calculate_level(400), 2);
        assert_eq!(UserProgress::calculate_level(900), 3);
    }

    #[test]
    fn test_user_progress_xp_addition() {
        let mut progress = UserProgress {
            id: None,
            xp_total: 100,
            level: 1,
            title: "Novice".to_string(),
            owner: Thing::from(("user".to_string(), "test".to_string())),
            device_id: "device1".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert_eq!(progress.level, 1);
        progress.add_xp(300);
        assert_eq!(progress.xp_total, 400);
        assert_eq!(progress.level, 2);
    }

    #[test]
    fn test_achievement_unlock() {
        let mut achievement = Achievement {
            id: None,
            name: "First Quest".to_string(),
            description: "Complete your first quest".to_string(),
            icon: "🏆".to_string(),
            unlocked_at: None,
            owner: Thing::from(("user".to_string(), "test".to_string())),
            device_id: "device1".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert!(!achievement.is_unlocked());
        achievement.unlock();
        assert!(achievement.is_unlocked());
    }

    #[test]
    fn test_streak_increment() {
        let mut streak = Streak {
            id: None,
            streak_type: StreakType::DailyCheckin,
            current_count: 5,
            longest_count: 5,
            last_activity: Utc::now(),
            started_at: Utc::now(),
            owner: Thing::from(("user".to_string(), "test".to_string())),
            device_id: "device1".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        streak.increment();
        assert_eq!(streak.current_count, 6);
        assert_eq!(streak.longest_count, 6);

        streak.reset();
        assert_eq!(streak.current_count, 0);
    }
}
