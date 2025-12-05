use crate::error::AppError;
use crate::models::{Operation, OperationBatch, OperationStatus, OperationType};
use std::path::PathBuf;
use tauri::Manager;

fn get_db_path(app: &tauri::AppHandle) -> PathBuf {
    app.path()
        .app_data_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("fileog.db")
}

#[tauri::command]
pub async fn get_operation_history(
    app: tauri::AppHandle,
    limit: Option<usize>,
) -> Result<Vec<OperationBatch>, AppError> {
    use rusqlite::Connection;

    let db_path = get_db_path(&app);

    if !db_path.exists() {
        return Ok(Vec::new());
    }

    let conn = Connection::open(&db_path)?;

    let limit = limit.unwrap_or(50);

    let mut stmt = conn.prepare(
        "SELECT id, batch_id, operation_type, source_path, destination_path, 
                original_name, new_name, timestamp, status, backup_path 
         FROM operations 
         ORDER BY timestamp DESC 
         LIMIT ?",
    )?;

    let operations: Vec<Operation> = stmt
        .query_map([limit], |row| {
            Ok(Operation {
                id: row.get(0)?,
                operation_type: match row.get::<_, String>(2)?.as_str() {
                    "move" => OperationType::Move,
                    "copy" => OperationType::Copy,
                    "rename" => OperationType::Rename,
                    "delete" => OperationType::Delete,
                    _ => OperationType::Move,
                },
                source_path: PathBuf::from(row.get::<_, String>(3)?),
                destination_path: row.get::<_, Option<String>>(4)?.map(PathBuf::from),
                original_name: row.get(5)?,
                new_name: row.get(6)?,
                timestamp: row.get(7)?,
                status: OperationStatus::Completed,
                batch_id: row.get(1)?,
                backup_path: row.get::<_, Option<String>>(9)?.map(PathBuf::from),
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

    // Group by batch_id
    use std::collections::HashMap;
    let mut batches: HashMap<String, Vec<Operation>> = HashMap::new();

    for op in operations {
        let batch_id = op.batch_id.clone().unwrap_or_else(|| op.id.clone());
        batches.entry(batch_id).or_default().push(op);
    }

    let result: Vec<OperationBatch> = batches
        .into_iter()
        .map(|(id, ops)| {
            let timestamp = ops.first().map(|o| o.timestamp).unwrap_or(0);
            OperationBatch {
                id,
                operations: ops,
                created_at: timestamp,
                description: String::new(),
            }
        })
        .collect();

    Ok(result)
}

#[tauri::command]
pub async fn undo_operations(
    app: tauri::AppHandle,
    steps: usize,
) -> Result<Vec<Operation>, AppError> {
    use rusqlite::Connection;

    let db_path = get_db_path(&app);

    if !db_path.exists() {
        return Ok(Vec::new());
    }

    let conn = Connection::open(&db_path)?;

    // Get the most recent completed operations to undo
    let mut stmt = conn.prepare(
        "SELECT id, batch_id, operation_type, source_path, destination_path,
                original_name, new_name, timestamp, status, backup_path
         FROM operations
         WHERE status = 'completed'
         ORDER BY timestamp DESC
         LIMIT ?",
    )?;

    let operations: Vec<Operation> = stmt
        .query_map([steps], |row| {
            Ok(Operation {
                id: row.get(0)?,
                operation_type: match row.get::<_, String>(2)?.as_str() {
                    "move" => OperationType::Move,
                    "copy" => OperationType::Copy,
                    "rename" => OperationType::Rename,
                    "delete" => OperationType::Delete,
                    _ => OperationType::Move,
                },
                source_path: PathBuf::from(row.get::<_, String>(3)?),
                destination_path: row.get::<_, Option<String>>(4)?.map(PathBuf::from),
                original_name: row.get(5)?,
                new_name: row.get(6)?,
                timestamp: row.get(7)?,
                status: OperationStatus::Completed,
                batch_id: row.get(1)?,
                backup_path: row.get::<_, Option<String>>(9)?.map(PathBuf::from),
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

    let mut undone = Vec::new();

    for op in operations {
        let result = match op.operation_type {
            OperationType::Move => {
                // Move back: dest -> source
                if let Some(dest) = &op.destination_path {
                    std::fs::rename(dest, &op.source_path)
                } else {
                    continue;
                }
            }
            OperationType::Copy => {
                // Delete the copy
                if let Some(dest) = &op.destination_path {
                    std::fs::remove_file(dest)
                } else {
                    continue;
                }
            }
            OperationType::Rename => {
                // Rename back: dest -> source
                if let Some(dest) = &op.destination_path {
                    std::fs::rename(dest, &op.source_path)
                } else {
                    continue;
                }
            }
            OperationType::Delete => {
                // Restore from backup if available
                if let Some(backup) = &op.backup_path {
                    if backup.exists() {
                        std::fs::rename(backup, &op.source_path)
                    } else {
                        continue;
                    }
                } else {
                    continue;
                }
            }
        };

        if result.is_ok() {
            // Mark as undone in database
            let _ = conn.execute(
                "UPDATE operations SET status = 'undone' WHERE id = ?",
                [&op.id],
            );
            undone.push(op);
        }
    }

    Ok(undone)
}

#[tauri::command]
pub async fn clear_history(app: tauri::AppHandle) -> Result<(), AppError> {
    use rusqlite::Connection;

    let db_path = get_db_path(&app);

    if db_path.exists() {
        let conn = Connection::open(&db_path)?;
        conn.execute("DELETE FROM operations", [])?;
    }

    Ok(())
}

pub fn init_database(app: &tauri::AppHandle) -> Result<(), AppError> {
    use rusqlite::Connection;

    let db_path = get_db_path(app);

    // Ensure parent directory exists
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let conn = Connection::open(&db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS operations (
            id TEXT PRIMARY KEY,
            batch_id TEXT,
            operation_type TEXT NOT NULL,
            source_path TEXT NOT NULL,
            destination_path TEXT,
            original_name TEXT,
            new_name TEXT,
            timestamp INTEGER NOT NULL,
            status TEXT NOT NULL,
            backup_path TEXT
        )",
        [],
    )?;

    Ok(())
}
