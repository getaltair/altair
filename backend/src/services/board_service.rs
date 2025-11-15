use tonic::{Request, Response, Status};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::quest_board::quest_board_service_server::QuestBoardService;
use crate::quest_board::*;
use crate::database::Database;

pub mod quest_board {
    tonic::include_proto!("altair.guidance");
}

pub struct BoardService {
    db: Database,
}

impl BoardService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
}

#[tonic::async_trait]
impl QuestBoardService for BoardService {
    async fn create_quest(
        &self,
        request: Request<CreateQuestRequest>,
    ) -> Result<Response<QuestResponse>, Status> {
        let req = request.into_inner();
        
        // Input validation
        if req.title.is_empty() {
            return Err(Status::invalid_argument("Title cannot be empty"));
        }
        if req.energy_points < 1 || req.energy_points > 5 {
            return Err(Status::invalid_argument("Energy points must be between 1 and 5"));
        }
        
        // Sanitize input
        let title = sanitize_input(&req.title);
        let description = req.description.as_ref().map(|d| sanitize_input(d));
        
        let quest_id = Uuid::new_v4().to_string();
        let now = Utc::now();
        
        let quest = Quest {
            id: quest_id.clone(),
            epic_id: if req.epic_id.is_empty() { None } else { Some(req.epic_id) },
            title,
            description,
            energy_points: req.energy_points,
            column: req.column,
            created_at: Some(prost_types::Timestamp {
                seconds: now.timestamp(),
                nanos: now.timestamp_subsec_nanos() as i32,
            }),
            updated_at: None,
            completed_at: None,
            tags: req.tags.iter().map(|t| sanitize_input(t)).collect(),
            assignee_id: if req.assignee_id.is_empty() { None } else { Some(req.assignee_id) },
            subquests: vec![],
            is_archived: false,
        };
        
        // Save to database
        match self.db.create_quest(&quest).await {
            Ok(_) => Ok(Response::new(QuestResponse {
                quest: Some(quest),
                success: true,
                error_message: String::new(),
            })),
            Err(e) => Err(Status::internal(format!("Failed to create quest: {}", e))),
        }
    }

    async fn get_quest(
        &self,
        request: Request<GetQuestRequest>,
    ) -> Result<Response<QuestResponse>, Status> {
        let req = request.into_inner();
        
        match self.db.get_quest(&req.id).await {
            Ok(Some(quest)) => Ok(Response::new(QuestResponse {
                quest: Some(quest),
                success: true,
                error_message: String::new(),
            })),
            Ok(None) => Err(Status::not_found("Quest not found")),
            Err(e) => Err(Status::internal(format!("Database error: {}", e))),
        }
    }

    async fn update_quest(
        &self,
        request: Request<UpdateQuestRequest>,
    ) -> Result<Response<QuestResponse>, Status> {
        let req = request.into_inner();
        
        // Input validation
        if req.title.is_empty() {
            return Err(Status::invalid_argument("Title cannot be empty"));
        }
        
        // Sanitize input
        let title = sanitize_input(&req.title);
        let description = req.description.as_ref().map(|d| sanitize_input(d));
        
        // Get existing quest
        let mut quest = match self.db.get_quest(&req.id).await {
            Ok(Some(q)) => q,
            Ok(None) => return Err(Status::not_found("Quest not found")),
            Err(e) => return Err(Status::internal(format!("Database error: {}", e))),
        };
        
        // Update fields
        quest.title = title;
        quest.description = description;
        quest.energy_points = req.energy_points;
        quest.tags = req.tags.iter().map(|t| sanitize_input(t)).collect();
        if !req.assignee_id.is_empty() {
            quest.assignee_id = Some(req.assignee_id);
        }
        
        let now = Utc::now();
        quest.updated_at = Some(prost_types::Timestamp {
            seconds: now.timestamp(),
            nanos: now.timestamp_subsec_nanos() as i32,
        });
        
        // Save to database
        match self.db.update_quest(&quest).await {
            Ok(_) => Ok(Response::new(QuestResponse {
                quest: Some(quest),
                success: true,
                error_message: String::new(),
            })),
            Err(e) => Err(Status::internal(format!("Failed to update quest: {}", e))),
        }
    }

    async fn delete_quest(
        &self,
        request: Request<DeleteQuestRequest>,
    ) -> Result<Response<DeleteQuestResponse>, Status> {
        let req = request.into_inner();
        
        match self.db.delete_quest(&req.id).await {
            Ok(_) => Ok(Response::new(DeleteQuestResponse {
                success: true,
                error_message: String::new(),
            })),
            Err(e) => Err(Status::internal(format!("Failed to delete quest: {}", e))),
        }
    }

    async fn list_quests(
        &self,
        request: Request<ListQuestsRequest>,
    ) -> Result<Response<ListQuestsResponse>, Status> {
        let req = request.into_inner();
        
        match self.db.list_quests(&req).await {
            Ok(quests) => Ok(Response::new(ListQuestsResponse {
                quests,
                success: true,
                error_message: String::new(),
            })),
            Err(e) => Err(Status::internal(format!("Database error: {}", e))),
        }
    }

    async fn move_quest(
        &self,
        request: Request<MoveQuestRequest>,
    ) -> Result<Response<QuestResponse>, Status> {
        let req = request.into_inner();
        
        // WIP=1 enforcement for In-Progress column
        if req.target_column == QuestColumn::QuestColumnInProgress as i32 {
            match self.db.get_quests_in_progress().await {
                Ok(in_progress) => {
                    if !in_progress.is_empty() && in_progress[0].id != req.id {
                        // Move current item out first
                        if let Err(e) = self.db.move_quest(&in_progress[0].id, QuestColumn::QuestColumnNextUp as i32).await {
                            return Err(Status::internal(format!("Failed to move current quest: {}", e)));
                        }
                    }
                }
                Err(e) => return Err(Status::internal(format!("Database error: {}", e))),
            }
        }
        
        match self.db.move_quest(&req.id, req.target_column).await {
            Ok(quest) => Ok(Response::new(QuestResponse {
                quest: Some(quest),
                success: true,
                error_message: String::new(),
            })),
            Err(e) => Err(Status::internal(format!("Failed to move quest: {}", e))),
        }
    }

    async fn create_epic(
        &self,
        request: Request<CreateEpicRequest>,
    ) -> Result<Response<EpicResponse>, Status> {
        let req = request.into_inner();
        
        if req.title.is_empty() {
            return Err(Status::invalid_argument("Title cannot be empty"));
        }
        
        let title = sanitize_input(&req.title);
        let description = req.description.as_ref().map(|d| sanitize_input(d));
        
        let epic_id = Uuid::new_v4().to_string();
        let now = Utc::now();
        
        let epic = Epic {
            id: epic_id.clone(),
            title,
            description,
            created_at: Some(prost_types::Timestamp {
                seconds: now.timestamp(),
                nanos: now.timestamp_subsec_nanos() as i32,
            }),
            updated_at: None,
            tags: req.tags.iter().map(|t| sanitize_input(t)).collect(),
            assignee_id: if req.assignee_id.is_empty() { None } else { Some(req.assignee_id) },
        };
        
        match self.db.create_epic(&epic).await {
            Ok(_) => Ok(Response::new(EpicResponse {
                epic: Some(epic),
                success: true,
                error_message: String::new(),
            })),
            Err(e) => Err(Status::internal(format!("Failed to create epic: {}", e))),
        }
    }

    async fn get_epic(
        &self,
        request: Request<GetEpicRequest>,
    ) -> Result<Response<EpicResponse>, Status> {
        let req = request.into_inner();
        
        match self.db.get_epic(&req.id).await {
            Ok(Some(epic)) => Ok(Response::new(EpicResponse {
                epic: Some(epic),
                success: true,
                error_message: String::new(),
            })),
            Ok(None) => Err(Status::not_found("Epic not found")),
            Err(e) => Err(Status::internal(format!("Database error: {}", e))),
        }
    }

    async fn update_epic(
        &self,
        request: Request<UpdateEpicRequest>,
    ) -> Result<Response<EpicResponse>, Status> {
        let req = request.into_inner();
        
        if req.title.is_empty() {
            return Err(Status::invalid_argument("Title cannot be empty"));
        }
        
        let mut epic = match self.db.get_epic(&req.id).await {
            Ok(Some(e)) => e,
            Ok(None) => return Err(Status::not_found("Epic not found")),
            Err(e) => return Err(Status::internal(format!("Database error: {}", e))),
        };
        
        epic.title = sanitize_input(&req.title);
        epic.description = req.description.as_ref().map(|d| sanitize_input(d));
        epic.tags = req.tags.iter().map(|t| sanitize_input(t)).collect();
        if !req.assignee_id.is_empty() {
            epic.assignee_id = Some(req.assignee_id);
        }
        
        let now = Utc::now();
        epic.updated_at = Some(prost_types::Timestamp {
            seconds: now.timestamp(),
            nanos: now.timestamp_subsec_nanos() as i32,
        });
        
        match self.db.update_epic(&epic).await {
            Ok(_) => Ok(Response::new(EpicResponse {
                epic: Some(epic),
                success: true,
                error_message: String::new(),
            })),
            Err(e) => Err(Status::internal(format!("Failed to update epic: {}", e))),
        }
    }

    async fn list_epics(
        &self,
        request: Request<ListEpicsRequest>,
    ) -> Result<Response<ListEpicsResponse>, Status> {
        let _req = request.into_inner();
        
        match self.db.list_epics().await {
            Ok(epics) => Ok(Response::new(ListEpicsResponse {
                epics,
                success: true,
                error_message: String::new(),
            })),
            Err(e) => Err(Status::internal(format!("Database error: {}", e))),
        }
    }

    async fn create_subquest(
        &self,
        request: Request<CreateSubquestRequest>,
    ) -> Result<Response<SubquestResponse>, Status> {
        let req = request.into_inner();
        
        if req.title.is_empty() {
            return Err(Status::invalid_argument("Title cannot be empty"));
        }
        if req.energy_points < 1 || req.energy_points > 5 {
            return Err(Status::invalid_argument("Energy points must be between 1 and 5"));
        }
        
        let title = sanitize_input(&req.title);
        let description = req.description.as_ref().map(|d| sanitize_input(d));
        
        let subquest_id = Uuid::new_v4().to_string();
        let now = Utc::now();
        
        let subquest = Subquest {
            id: subquest_id.clone(),
            quest_id: req.quest_id,
            title,
            description,
            energy_points: req.energy_points,
            created_at: Some(prost_types::Timestamp {
                seconds: now.timestamp(),
                nanos: now.timestamp_subsec_nanos() as i32,
            }),
            updated_at: None,
            completed_at: None,
            tags: req.tags.iter().map(|t| sanitize_input(t)).collect(),
            assignee_id: if req.assignee_id.is_empty() { None } else { Some(req.assignee_id) },
            is_completed: false,
        };
        
        match self.db.create_subquest(&subquest).await {
            Ok(_) => Ok(Response::new(SubquestResponse {
                subquest: Some(subquest),
                success: true,
                error_message: String::new(),
            })),
            Err(e) => Err(Status::internal(format!("Failed to create subquest: {}", e))),
        }
    }

    async fn update_subquest(
        &self,
        request: Request<UpdateSubquestRequest>,
    ) -> Result<Response<SubquestResponse>, Status> {
        let req = request.into_inner();
        
        let mut subquest = match self.db.get_subquest(&req.id).await {
            Ok(Some(s)) => s,
            Ok(None) => return Err(Status::not_found("Subquest not found")),
            Err(e) => return Err(Status::internal(format!("Database error: {}", e))),
        };
        
        subquest.title = sanitize_input(&req.title);
        subquest.description = req.description.as_ref().map(|d| sanitize_input(d));
        subquest.energy_points = req.energy_points;
        subquest.tags = req.tags.iter().map(|t| sanitize_input(t)).collect();
        subquest.is_completed = req.is_completed;
        if !req.assignee_id.is_empty() {
            subquest.assignee_id = Some(req.assignee_id);
        }
        
        let now = Utc::now();
        subquest.updated_at = Some(prost_types::Timestamp {
            seconds: now.timestamp(),
            nanos: now.timestamp_subsec_nanos() as i32,
        });
        
        match self.db.update_subquest(&subquest).await {
            Ok(_) => Ok(Response::new(SubquestResponse {
                subquest: Some(subquest),
                success: true,
                error_message: String::new(),
            })),
            Err(e) => Err(Status::internal(format!("Failed to update subquest: {}", e))),
        }
    }

    async fn list_subquests(
        &self,
        request: Request<ListSubquestsRequest>,
    ) -> Result<Response<ListSubquestsResponse>, Status> {
        let req = request.into_inner();
        
        match self.db.list_subquests(&req.quest_id).await {
            Ok(subquests) => Ok(Response::new(ListSubquestsResponse {
                subquests,
                success: true,
                error_message: String::new(),
            })),
            Err(e) => Err(Status::internal(format!("Database error: {}", e))),
        }
    }

    async fn bulk_move_quests(
        &self,
        request: Request<BulkMoveQuestsRequest>,
    ) -> Result<Response<BulkOperationResponse>, Status> {
        let req = request.into_inner();
        
        let mut success_count = 0;
        let mut failure_count = 0;
        let mut errors = Vec::new();
        
        for quest_id in req.quest_ids {
            match self.db.move_quest(&quest_id, req.target_column).await {
                Ok(_) => success_count += 1,
                Err(e) => {
                    failure_count += 1;
                    errors.push(format!("Quest {}: {}", quest_id, e));
                }
            }
        }
        
        Ok(Response::new(BulkOperationResponse {
            success_count,
            failure_count,
            errors,
        }))
    }

    async fn bulk_delete_quests(
        &self,
        request: Request<BulkDeleteQuestsRequest>,
    ) -> Result<Response<BulkOperationResponse>, Status> {
        let req = request.into_inner();
        
        let mut success_count = 0;
        let mut failure_count = 0;
        let mut errors = Vec::new();
        
        for quest_id in req.quest_ids {
            match self.db.delete_quest(&quest_id).await {
                Ok(_) => success_count += 1,
                Err(e) => {
                    failure_count += 1;
                    errors.push(format!("Quest {}: {}", quest_id, e));
                }
            }
        }
        
        Ok(Response::new(BulkOperationResponse {
            success_count,
            failure_count,
            errors,
        }))
    }

    async fn bulk_archive_quests(
        &self,
        request: Request<BulkArchiveQuestsRequest>,
    ) -> Result<Response<BulkOperationResponse>, Status> {
        let req = request.into_inner();
        
        let mut success_count = 0;
        let mut failure_count = 0;
        let mut errors = Vec::new();
        
        for quest_id in req.quest_ids {
            match self.db.archive_quest(&quest_id).await {
                Ok(_) => success_count += 1,
                Err(e) => {
                    failure_count += 1;
                    errors.push(format!("Quest {}: {}", quest_id, e));
                }
            }
        }
        
        Ok(Response::new(BulkOperationResponse {
            success_count,
            failure_count,
            errors,
        }))
    }

    async fn archive_old_quests(
        &self,
        request: Request<ArchiveOldQuestsRequest>,
    ) -> Result<Response<ArchiveOldQuestsResponse>, Status> {
        let req = request.into_inner();
        
        match self.db.archive_old_quests(req.days_old).await {
            Ok(count) => Ok(Response::new(ArchiveOldQuestsResponse {
                archived_count: count as i32,
                success: true,
                error_message: String::new(),
            })),
            Err(e) => Err(Status::internal(format!("Failed to archive quests: {}", e))),
        }
    }

    async fn get_quests_by_column(
        &self,
        request: Request<GetQuestsByColumnRequest>,
    ) -> Result<Response<ListQuestsResponse>, Status> {
        let req = request.into_inner();
        
        match self.db.get_quests_by_column(req.column).await {
            Ok(quests) => Ok(Response::new(ListQuestsResponse {
                quests,
                success: true,
                error_message: String::new(),
            })),
            Err(e) => Err(Status::internal(format!("Database error: {}", e))),
        }
    }

    async fn health_check(
        &self,
        _request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        Ok(Response::new(HealthCheckResponse {
            healthy: true,
            version: env!("CARGO_PKG_VERSION").to_string(),
            uptime_seconds: 0, // TODO: Track actual uptime
        }))
    }
}

/// Sanitize input to prevent XSS attacks
fn sanitize_input(input: &str) -> String {
    // Remove HTML tags and escape special characters
    input
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
        .replace('/', "&#x2F;")
        .trim()
        .to_string()
}

