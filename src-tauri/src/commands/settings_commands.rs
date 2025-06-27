use crate::menu;
use crate::shortcuts;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: String,
    pub language: String,
    pub default_format: String,
    pub default_path: Option<String>,
    pub new_note_shortcut: String,
    pub close_tab_shortcut: String,
    pub toggle_window_shortcut: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: "light".to_string(),
            language: "en".to_string(),
            default_format: "txt".to_string(),
            default_path: None,
            new_note_shortcut: "CmdOrCtrl+Option+T".to_string(),
            close_tab_shortcut: "CmdOrCtrl+Option+Y".to_string(),
            toggle_window_shortcut: "CmdOrCtrl+Option+U".to_string(),
        }
    }
}

/// Helper function to get the path to the settings.json file
fn get_settings_path(app: &tauri::AppHandle) -> anyhow::Result<PathBuf> {
    let config_dir = app
        .path_resolver()
        .app_config_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find app config directory"))?;

    // Create the config directory if it doesn't exist
    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir)?;
    }

    Ok(config_dir.join("settings.json"))
}

/// Loads application settings from settings.json
#[tauri::command]
pub async fn load_settings(app: tauri::AppHandle) -> Result<AppSettings, String> {
    let path = get_settings_path(&app).map_err(|e| e.to_string())?;

    if !path.exists() {
        return Ok(AppSettings::default());
    }

    match fs::read_to_string(&path).await {
        Ok(content) => serde_json::from_str(&content).map_err(|e| e.to_string()),
        Err(_) => {
            // If reading fails (e.g., file is corrupted or just created), return default
            Ok(AppSettings::default())
        }
    }
}

/// Saves application settings to settings.json
#[tauri::command]
pub async fn save_settings(app: tauri::AppHandle, settings: AppSettings) -> Result<(), String> {
    let path = get_settings_path(&app).map_err(|e| e.to_string())?;
    let json_content = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;

    fs::write(&path, json_content)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// Updates the shortcut for new note or close tab
#[tauri::command]
pub async fn update_shortcut(
    app: tauri::AppHandle,
    shortcut: String,
    action: Option<String>,
) -> Result<(), String> {
    match action.as_deref() {
        Some("close_tab") => shortcuts::register_close_tab_shortcut(&app, &shortcut),
        Some("toggle_window") => shortcuts::register_toggle_window_shortcut(&app, &shortcut),
        Some("new_note") | None => shortcuts::register_new_note_shortcut(&app, &shortcut),
        Some(other) => {
            eprintln!("Unknown shortcut action: {}", other);
        }
    }
    Ok(())
}

/// Updates the system tray menu with recent files
#[tauri::command]
pub async fn update_tray_menu(
    app: tauri::AppHandle,
    recent_files: Vec<String>,
) -> Result<(), String> {
    menu::update_tray_menu_with_recent_files(&app, recent_files);
    Ok(())
}

/// Opens a directory selection dialog and returns the selected path
#[tauri::command]
pub async fn select_directory() -> Result<Option<String>, String> {
    use tauri::api::dialog::blocking::FileDialogBuilder;

    let dialog = FileDialogBuilder::new().set_title("Select Default Directory");

    match dialog.pick_folder() {
        Some(path) => Ok(Some(path.to_string_lossy().to_string())),
        None => Ok(None),
    }
}
