use crate::error::AppError;
use crate::models::AppSettings;
use crate::services::llm::{LlmService, ClassificationResult};
use tauri::Manager;

#[derive(serde::Deserialize)]
pub struct ClassifyRequest {
    pub files: Vec<FileToClassify>,
}

#[derive(serde::Deserialize)]
pub struct FileToClassify {
    pub path: String,
    pub name: String,
    pub extension: String,
    pub size: u64,
}

fn read_settings(app: &tauri::AppHandle) -> Result<AppSettings, AppError> {
    let path = app.path()
        .app_config_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
        .join("settings.json");

    if path.exists() {
        let content = std::fs::read_to_string(&path)?;
        let settings: AppSettings = serde_json::from_str(&content)?;
        Ok(settings)
    } else {
        Ok(AppSettings::default())
    }
}

#[tauri::command]
pub async fn classify_files(
    app: tauri::AppHandle,
    request: ClassifyRequest,
) -> Result<Vec<ClassificationResult>, AppError> {
    let settings = read_settings(&app)?;

    if !settings.llm.enabled {
        return Err(AppError::LlmError("LLM is not enabled".to_string()));
    }

    let config = settings.llm.config;

    if config.api_key.is_empty() && config.provider != "ollama" {
        return Err(AppError::LlmError("API key is not configured".to_string()));
    }

    let categories = get_category_names(&app)?;
    let service = LlmService::new(config);

    let files: Vec<(String, String, u64)> = request
        .files
        .into_iter()
        .map(|f| (f.name, f.extension, f.size))
        .collect();

    let custom_prompt = if !settings.prompts.filename_prompt.is_empty() {
        Some(settings.prompts.filename_prompt.as_str())
    } else {
        None
    };

    let results = service
        .classify_files_batch(files, &categories, custom_prompt)
        .await?;

    Ok(results)
}

#[tauri::command]
pub async fn classify_single_file(
    app: tauri::AppHandle,
    file: FileToClassify,
) -> Result<ClassificationResult, AppError> {
    let settings = read_settings(&app)?;

    if !settings.llm.enabled {
        return Err(AppError::LlmError("LLM is not enabled".to_string()));
    }

    let config = settings.llm.config;

    if config.api_key.is_empty() && config.provider != "ollama" {
        return Err(AppError::LlmError("API key is not configured".to_string()));
    }

    let categories = get_category_names(&app)?;
    let service = LlmService::new(config);

    let custom_prompt = if !settings.prompts.filename_prompt.is_empty() {
        Some(settings.prompts.filename_prompt.as_str())
    } else {
        None
    };

    service
        .classify_file(&file.name, &file.extension, file.size, &categories, custom_prompt)
        .await
}

fn get_category_names(app: &tauri::AppHandle) -> Result<Vec<String>, AppError> {
    let config_dir = app.path()
        .app_config_dir()
        .map_err(|e| AppError::Config(e.to_string()))?;

    let categories_path = config_dir.join("categories.json");

    if categories_path.exists() {
        let content = std::fs::read_to_string(&categories_path)?;

        #[derive(serde::Deserialize)]
        struct Category {
            name: String,
        }

        let categories: Vec<Category> = serde_json::from_str(&content)?;
        Ok(categories.into_iter().map(|c| c.name).collect())
    } else {
        Ok(vec![
            "文档".to_string(),
            "图片".to_string(),
            "视频".to_string(),
            "音乐".to_string(),
            "代码".to_string(),
            "压缩包".to_string(),
            "其他".to_string(),
        ])
    }
}

#[tauri::command]
pub async fn test_llm_connection(
    app: tauri::AppHandle,
) -> Result<String, AppError> {
    let settings = read_settings(&app)?;
    let config = settings.llm.config;

    let service = LlmService::new(config);

    let result = service
        .classify_file(
            "test.txt",
            "txt",
            1024,
            &["documents".to_string(), "others".to_string()],
            None,
        )
        .await?;

    Ok(format!(
        "Connection successful! Test result: {} (confidence: {:.0}%)",
        result.suggested_category,
        result.confidence * 100.0
    ))
}
