//! Inventory domain types - Item and location tracking entities

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use surrealdb::sql::Thing;

use super::enums::{EntityStatus, ItemStatus, ReservationStatus};

/// Item - Physical or digital inventory item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: Option<Thing>,
    pub name: String,
    pub description: Option<String>,
    pub quantity: i32,
    pub status: ItemStatus,
    pub category: Option<String>,
    pub custom_fields: Option<JsonValue>, // flexible metadata
    pub owner: Thing,
    pub device_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Item {
    /// Check if item is available for use
    pub fn is_available(&self) -> bool {
        matches!(self.status, ItemStatus::Available) && self.quantity > 0
    }

    /// Check if item needs restocking
    pub fn needs_restock(&self, threshold: i32) -> bool {
        self.quantity < threshold
    }
}

/// Location - Physical or logical storage location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub id: Option<Thing>,
    pub name: String,
    pub description: Option<String>,
    pub geo: Option<GeoPoint>, // optional geographic coordinates
    pub status: EntityStatus,
    pub owner: Thing,
    pub device_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Geographic point (latitude, longitude)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoPoint {
    pub latitude: f64,
    pub longitude: f64,
}

/// Reservation - Temporary allocation of items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reservation {
    pub id: Option<Thing>,
    pub quantity: i32,
    pub status: ReservationStatus,
    pub reserved_at: DateTime<Utc>,
    pub released_at: Option<DateTime<Utc>>,
    pub owner: Thing,
    pub device_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Reservation {
    /// Check if reservation is currently active
    pub fn is_active(&self) -> bool {
        matches!(self.status, ReservationStatus::InUse)
    }

    /// Release the reservation
    pub fn release(&mut self) {
        self.status = ReservationStatus::Released;
        self.released_at = Some(Utc::now());
    }
}

/// MaintenanceSchedule - Recurring maintenance tasks for items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceSchedule {
    pub id: Option<Thing>,
    pub task_name: String,
    pub interval: String, // duration string (e.g., "30d", "1w")
    pub last_performed: Option<DateTime<Utc>>,
    pub next_due: DateTime<Utc>,
    pub notes: Option<String>,
    pub notify_days_before: Option<i32>,
    pub owner: Thing,
    pub device_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl MaintenanceSchedule {
    /// Check if maintenance is overdue
    pub fn is_overdue(&self) -> bool {
        self.next_due < Utc::now()
    }

    /// Check if notification should be sent
    pub fn should_notify(&self) -> bool {
        if let Some(days) = self.notify_days_before {
            let notify_date = self.next_due - chrono::Duration::days(days as i64);
            Utc::now() >= notify_date
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_availability() {
        let item = Item {
            id: None,
            name: "Test Item".to_string(),
            description: None,
            quantity: 5,
            status: ItemStatus::Available,
            category: None,
            custom_fields: None,
            owner: Thing::from(("user".to_string(), "test".to_string())),
            device_id: "device1".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert!(item.is_available());
        assert!(item.needs_restock(10)); // quantity 5 < threshold 10
        assert!(!item.needs_restock(3)); // quantity 5 >= threshold 3
    }

    #[test]
    fn test_reservation_lifecycle() {
        let mut reservation = Reservation {
            id: None,
            quantity: 2,
            status: ReservationStatus::InUse,
            reserved_at: Utc::now(),
            released_at: None,
            owner: Thing::from(("user".to_string(), "test".to_string())),
            device_id: "device1".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert!(reservation.is_active());

        reservation.release();
        assert!(!reservation.is_active());
        assert!(reservation.released_at.is_some());
    }
}
