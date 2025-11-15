use tonic::transport::Server;
use tracing::{info, Level};
use tracing_subscriber;

mod quest_board {
    tonic::include_proto!("altair.guidance");
}

mod database;
mod services;

use services::board_service::{BoardService, quest_board::quest_board_service_server::QuestBoardServiceServer};
use database::Database;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    // Initialize database
    let db_path = PathBuf::from("./data/altair.db");
    let db = Database::new(db_path).await?;
    
    // Create service
    let board_service = BoardService::new(db);
    
    // Start gRPC server
    let addr = "127.0.0.1:50051".parse()?;
    info!("Quest Board gRPC server listening on {}", addr);
    
    Server::builder()
        .add_service(QuestBoardServiceServer::new(board_service))
        .serve(addr)
        .await?;

    Ok(())
}

