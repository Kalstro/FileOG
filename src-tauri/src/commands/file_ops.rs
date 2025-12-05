use crate::error::AppError;
use crate::models::{Operation, OperationType, OperationStatus, PlannedOperation};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::ipc::Channel;
use tauri::Manager;
use uuid::Uuid;
use chrono::Utc;

#[derive(Debug, Clone, Serialize)]
pub struct OperationProgress {
    pub event: String,
    pub current_file: Option<String>,
    pub completed_count: usize,
    pub total_count: usize,
    pub percentage: f32,
}

fn get_db_path(app: &tauri::AppHandle) -> PathBuf {
    app.path()
        .app_data_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("fileog.db")
}

fn save_operation_to_db(app: &tauri::AppHandle, operation: &Operation) -> Result<(), AppError> {
    use rusqlite::Connection;

    let db_path = get_db_path(app);
    let conn = Connection::open(&db_path)?;

    let op_type = match operation.operation_type {
        OperationType::Move => "move",
        OperationType::Copy => "copy",
        OperationType::Rename => "rename",
        OperationType::Delete => "delete",
    };

    let status = match &operation.status {
        OperationStatus::Completed => "completed".to_string(),
        OperationStatus::Failed(e) => format!("failed:{}", e),
        OperationStatus::Pending => "pending".to_string(),
        OperationStatus::InProgress => "in_progress".to_string(),
        OperationStatus::Undone => "undone".to_string(),
    };

    conn.execute(
        "INSERT INTO operations (id, batch_id, operation_type, source_path, destination_path, original_name, new_name, timestamp, status, backup_path)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        (
            &operation.id,
            &operation.batch_id,
            op_type,
            operation.source_path.to_string_lossy().to_string(),
            operation.destination_path.as_ref().map(|p| p.to_string_lossy().to_string()),
            &operation.original_name,
            &operation.new_name,
            operation.timestamp,
            status,
            operation.backup_path.as_ref().map(|p| p.to_string_lossy().to_string()),
        ),
    )?;

    Ok(())
}

#[tauri::command]
pub async fn execute_operations(
    app: tauri::AppHandle,
    operations: Vec<PlannedOperation>,
    on_progress: Channel<OperationProgress>,
) -> Result<Vec<Operation>, AppError> {
    let total = operations.len();
    let mut results = Vec::new();
    let batch_id = Uuid::new_v4().to_string();

    for (index, planned) in operations.into_iter().enumerate() {
        let _ = on_progress.send(OperationProgress {
            event: "processing".to_string(),
            current_file: Some(planned.file_name.clone()),
            completed_count: index,
            total_count: total,
            percentage: (index as f32 / total as f32) * 100.0,
        });

        let result = match planned.operation_type {
            OperationType::Move => {
                std::fs::rename(&planned.source, &planned.destination)
                    .map_err(|e| AppError::Io(e))
            }
            OperationType::Copy => {
                std::fs::copy(&planned.source, &planned.destination)
                    .map(|_| ())
                    .map_err(|e| AppError::Io(e))
            }
            OperationType::Rename => {
                std::fs::rename(&planned.source, &planned.destination)
                    .map_err(|e| AppError::Io(e))
            }
            OperationType::Delete => {
                std::fs::remove_file(&planned.source)
                    .map_err(|e| AppError::Io(e))
            }
        };

        let destination_path = match planned.operation_type {
            OperationType::Delete => None,
            _ => Some(planned.destination),
        };

        let operation = Operation {
            id: Uuid::new_v4().to_string(),
            operation_type: planned.operation_type,
            source_path: planned.source,
            destination_path,
            original_name: Some(planned.file_name.clone()),
            new_name: None,
            timestamp: Utc::now().timestamp(),
            status: match &result {
                Ok(_) => OperationStatus::Completed,
                Err(e) => OperationStatus::Failed(e.to_string()),
            },
            batch_id: Some(batch_id.clone()),
            backup_path: None,
        };

        // Save to database
        if let Err(e) = save_operation_to_db(&app, &operation) {
            eprintln!("Failed to save operation to database: {:?}", e);
        }

        results.push(operation);
    }

    let _ = on_progress.send(OperationProgress {
        event: "completed".to_string(),
        current_file: None,
        completed_count: total,
        total_count: total,
        percentage: 100.0,
    });

    Ok(results)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateGroup {
    pub hash: String,
    pub files: Vec<PathBuf>,
    pub size: u64,
}

#[tauri::command]
pub async fn find_duplicates(
    files: Vec<crate::models::FileItem>,
    on_progress: Channel<OperationProgress>,
) -> Result<Vec<DuplicateGroup>, AppError> {
    use sha2::{Sha256, Digest};
    use std::collections::HashMap;
    use std::io::Read;

    let total = files.len();
    let mut hash_map: HashMap<String, Vec<PathBuf>> = HashMap::new();

    for (index, file) in files.iter().enumerate() {
        let _ = on_progress.send(OperationProgress {
            event: "hashing".to_string(),
            current_file: Some(file.name.clone()),
            completed_count: index,
            total_count: total,
            percentage: (index as f32 / total as f32) * 100.0,
        });

        // Calculate hash
        if let Ok(mut f) = std::fs::File::open(&file.path) {
            let mut hasher = Sha256::new();
            let mut buffer = [0u8; 8192];

            loop {
                match f.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(n) => hasher.update(&buffer[..n]),
                    Err(_) => break,
                }
            }

            let hash = format!("{:x}", hasher.finalize());
            hash_map
                .entry(hash)
                .or_insert_with(Vec::new)
                .push(file.path.clone());
        }
    }

    // Filter to only groups with duplicates
    let duplicates: Vec<DuplicateGroup> = hash_map
        .into_iter()
        .filter(|(_, paths)| paths.len() > 1)
        .map(|(hash, files)| {
            let size = std::fs::metadata(&files[0])
                .map(|m| m.len())
                .unwrap_or(0);
            DuplicateGroup { hash, files, size }
        })
        .collect();

    let _ = on_progress.send(OperationProgress {
        event: "completed".to_string(),
        current_file: None,
        completed_count: total,
        total_count: total,
        percentage: 100.0,
    });

    Ok(duplicates)
}
