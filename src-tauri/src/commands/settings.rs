use crate::error::AppError;
use crate::models::{AppSettings, Category, default_categories};
use std::path::PathBuf;
use tauri::Manager;

fn get_config_path(app: &tauri::AppHandle) -> PathBuf {
    app.path()
        .app_config_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("settings.json")
}

fn get_categories_path(app: &tauri::AppHandle) -> PathBuf {
    app.path()
        .app_config_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("categories.json")
}

/// Internal non-async version for use by other modules
pub fn get_settings_internal(app: &tauri::AppHandle) -> Result<AppSettings, AppError> {
    let path = get_config_path(app);

    if path.exists() {
        let content = std::fs::read_to_string(&path)?;
        let settings: AppSettings = serde_json::from_str(&content)?;
        Ok(settings)
    } else {
        Ok(AppSettings::default())
    }
}

#[tauri::command]
pub async fn get_settings(app: tauri::AppHandle) -> Result<AppSettings, AppError> {
    let path = get_config_path(&app);
    
    if path.exists() {
        let content = std::fs::read_to_string(&path)?;
        let settings: AppSettings = serde_json::from_str(&content)?;
        Ok(settings)
    } else {
        Ok(AppSettings::default())
    }
}

#[tauri::command]
pub async fn save_settings(app: tauri::AppHandle, settings: AppSettings) -> Result<(), AppError> {
    let path = get_config_path(&app);
    
    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    let content = serde_json::to_string_pretty(&settings)?;
    std::fs::write(&path, content)?;
    
    Ok(())
}

#[tauri::command]
pub async fn get_categories(app: tauri::AppHandle) -> Result<Vec<Category>, AppError> {
    let path = get_categories_path(&app);
    
    if path.exists() {
        let content = std::fs::read_to_string(&path)?;
        let categories: Vec<Category> = serde_json::from_str(&content)?;
        Ok(categories)
    } else {
        Ok(default_categories())
    }
}

#[tauri::command]
pub async fn save_categories(app: tauri::AppHandle, categories: Vec<Category>) -> Result<(), AppError> {
    let path = get_categories_path(&app);
    
    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    let content = serde_json::to_string_pretty(&categories)?;
    std::fs::write(&path, content)?;
    
    Ok(())
}
