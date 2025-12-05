use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileItem {
    pub id: String,
    pub path: PathBuf,
    pub name: String,
    pub extension: Option<String>,
    pub size: u64,
    pub file_type: FileType,
    pub hash: Option<String>,
    pub created_at: i64,
    pub modified_at: i64,
    pub category: Option<String>,
    pub metadata: FileMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FileType {
    Document,
    Image,
    Video,
    Audio,
    Archive,
    Code,
    Other,
}

impl FileType {
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            // Documents
            "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" | "txt" | "rtf" | "odt" | "ods" | "odp" => FileType::Document,
            // Images
            "jpg" | "jpeg" | "png" | "gif" | "webp" | "svg" | "bmp" | "ico" | "tiff" | "heic" => FileType::Image,
            // Videos
            "mp4" | "mkv" | "avi" | "mov" | "wmv" | "flv" | "webm" | "m4v" => FileType::Video,
            // Audio
            "mp3" | "flac" | "wav" | "aac" | "ogg" | "wma" | "m4a" => FileType::Audio,
            // Archives
            "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" | "xz" => FileType::Archive,
            // Code
            "js" | "ts" | "jsx" | "tsx" | "py" | "rs" | "go" | "java" | "c" | "cpp" | "h" | "hpp" | "cs" | "rb" | "php" | "swift" | "kt" | "scala" | "html" | "css" | "scss" | "json" | "yaml" | "yml" | "xml" | "md" | "sql" => FileType::Code,
            _ => FileType::Other,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FileMetadata {
    pub mime_type: Option<String>,
    pub dimensions: Option<(u32, u32)>,
    pub duration: Option<u64>,
    pub preview_text: Option<String>,
}
