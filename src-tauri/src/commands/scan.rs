use crate::error::AppError;
use crate::models::{FileItem, FileMetadata, FileType};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::ipc::Channel;
use uuid::Uuid;
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanOptions {
    pub path: PathBuf,
    pub recursive: bool,
    pub include_hidden: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct ScanProgress {
    pub event: String,
    pub current_file: Option<String>,
    pub scanned_count: usize,
    pub total_count: Option<usize>,
}

#[tauri::command]
pub async fn scan_directory(
    options: ScanOptions,
    on_progress: Channel<ScanProgress>,
) -> Result<Vec<FileItem>, AppError> {
    let mut files = Vec::new();
    let mut count = 0;

    // Send start event
    let _ = on_progress.send(ScanProgress {
        event: "started".to_string(),
        current_file: None,
        scanned_count: 0,
        total_count: None,
    });

    let walker = if options.recursive {
        WalkDir::new(&options.path)
    } else {
        WalkDir::new(&options.path).max_depth(1)
    };

    for entry in walker.into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        
        // Skip directories
        if path.is_dir() {
            continue;
        }

        // Skip hidden files if not included
        if !options.include_hidden {
            if let Some(name) = path.file_name() {
                if name.to_string_lossy().starts_with('.') {
                    continue;
                }
            }
        }

        let metadata = std::fs::metadata(path)?;
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        let extension = path
            .extension()
            .map(|e| e.to_string_lossy().to_string());
        
        let file_type = extension
            .as_ref()
            .map(|e| FileType::from_extension(e))
            .unwrap_or(FileType::Other);

        let file_item = FileItem {
            id: Uuid::new_v4().to_string(),
            path: path.to_path_buf(),
            name,
            extension,
            size: metadata.len(),
            file_type,
            hash: None,
            created_at: metadata
                .created()
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_secs() as i64)
                .unwrap_or(0),
            modified_at: metadata
                .modified()
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_secs() as i64)
                .unwrap_or(0),
            category: None,
            metadata: FileMetadata::default(),
        };

        files.push(file_item);
        count += 1;

        // Send progress every 10 files
        if count % 10 == 0 {
            let _ = on_progress.send(ScanProgress {
                event: "scanning".to_string(),
                current_file: Some(path.to_string_lossy().to_string()),
                scanned_count: count,
                total_count: None,
            });
        }
    }

    // Send completion event
    let _ = on_progress.send(ScanProgress {
        event: "completed".to_string(),
        current_file: None,
        scanned_count: count,
        total_count: Some(count),
    });

    Ok(files)
}
