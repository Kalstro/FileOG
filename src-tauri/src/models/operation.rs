use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    pub id: String,
    pub operation_type: OperationType,
    pub source_path: PathBuf,
    pub destination_path: Option<PathBuf>,
    pub original_name: Option<String>,
    pub new_name: Option<String>,
    pub timestamp: i64,
    pub status: OperationStatus,
    pub batch_id: Option<String>,
    pub backup_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OperationType {
    Move,
    Copy,
    Rename,
    Delete,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OperationStatus {
    Pending,
    InProgress,
    Completed,
    Failed(String),
    Undone,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationBatch {
    pub id: String,
    pub operations: Vec<Operation>,
    pub created_at: i64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlannedOperation {
    pub file_id: String,
    pub file_name: String,
    pub operation_type: OperationType,
    pub source: PathBuf,
    pub destination: PathBuf,
    pub category: Option<String>,
}
