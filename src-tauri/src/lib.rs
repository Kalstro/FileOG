pub mod error;
pub mod models;
pub mod commands;
pub mod services;

use commands::{scan, file_ops, settings, history, llm};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            // Initialize database
            if let Err(e) = history::init_database(app.handle()) {
                eprintln!("Failed to initialize database: {}", e);
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Scan commands
            scan::scan_directory,
            // File operation commands
            file_ops::execute_operations,
            file_ops::find_duplicates,
            // Settings commands
            settings::get_settings,
            settings::save_settings,
            settings::get_categories,
            settings::save_categories,
            // History commands
            history::get_operation_history,
            history::undo_operations,
            history::clear_history,
            // LLM commands
            llm::classify_files,
            llm::classify_single_file,
            llm::test_llm_connection,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
