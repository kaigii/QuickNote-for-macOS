use anyhow::Result;
use serde::{Deserialize, Serialize};
use tauri::api::dialog;
use tauri::Window;
use tokio::fs;

/// Payload structure for file operations
#[derive(Debug, Serialize, Deserialize)]
pub struct FilePayload {
    pub path: String,
    pub content: String,
}

/// Opens one or more files using a system dialog and returns their content
#[tauri::command]
pub async fn open_file(window: Window) -> Result<Option<Vec<FilePayload>>, String> {
    // Show file open dialog for multiple files
    let file_paths = dialog::blocking::FileDialogBuilder::new()
        .add_filter(
            "Text Files",
            &[
                "txt", "md", "json", "js", "ts", "vue", "rs", "py", "java", "cpp", "c", "h", "hpp",
                "cs", "php", "rb", "go", "swift", "kt", "scala", "r", "m", "pl", "sh", "bat",
                "ps1", "sql", "xml", "html", "css", "scss", "sass", "less", "yaml", "yml", "toml",
                "ini", "cfg", "conf", "log",
            ],
        )
        .add_filter("All Files", &["*"])
        .pick_files();

    match file_paths {
        Some(paths) => {
            let mut files_content = Vec::new();
            for path in paths {
                // Read file content asynchronously
                match fs::read_to_string(&path).await {
                    Ok(content) => {
                        let payload = FilePayload {
                            path: path.to_string_lossy().to_string(),
                            content,
                        };
                        files_content.push(payload);
                    }
                    Err(e) => {
                        let path_str = path.to_string_lossy();
                        eprintln!("Failed to read file {}: {}", path_str, e);
                        let _ = window.emit("error", format!("Failed to read file: {}", path_str));
                    }
                }
            }
            if files_content.is_empty() {
                Ok(None)
            } else {
                Ok(Some(files_content))
            }
        }
        None => Ok(None), // User cancelled the dialog
    }
}

/// Opens a specific file by path and returns its content
#[tauri::command]
pub async fn open_specific_file(path: String) -> Result<Option<FilePayload>, String> {
    // Check if file exists
    if !std::path::Path::new(&path).exists() {
        return Err(format!("File not found: {}", path));
    }

    // Read file content asynchronously
    match fs::read_to_string(&path).await {
        Ok(content) => {
            let payload = FilePayload {
                path: path.to_string(),
                content,
            };
            Ok(Some(payload))
        }
        Err(e) => Err(format!("Failed to read file: {}", e)),
    }
}

/// Saves content to a specified file path
#[tauri::command]
pub async fn save_file(path: String, content: String) -> Result<(), String> {
    fs::write(&path, content).await.map_err(|e| e.to_string())
}

/// Saves content to a file using a system save dialog
#[tauri::command]
pub async fn save_file_as(
    content: String,
    default_format: String,
    default_name: String,
) -> Result<Option<String>, String> {
    // Create filter based on default format
    let format_filter: (&str, &[&str]) = match default_format.as_str() {
        "txt" => ("Text Files", &["txt"] as &[&str]),
        "md" => ("Markdown Files", &["md", "markdown"] as &[&str]),
        "js" => ("JavaScript Files", &["js"] as &[&str]),
        "ts" => ("TypeScript Files", &["ts"] as &[&str]),
        "json" => ("JSON Files", &["json"] as &[&str]),
        "html" => ("HTML Files", &["html", "htm"] as &[&str]),
        "css" => ("CSS Files", &["css"] as &[&str]),
        _ => ("Text Files", &["txt"] as &[&str]), // Default fallback
    };

    // Show file save dialog with default filename
    let file_path = dialog::blocking::FileDialogBuilder::new()
        .set_file_name(&default_name)
        .add_filter(format_filter.0, format_filter.1)
        .add_filter(
            "Text Files",
            &[
                "txt", "md", "json", "js", "ts", "vue", "rs", "py", "java", "cpp", "c", "h", "hpp",
                "cs", "php", "rb", "go", "swift", "kt", "scala", "r", "m", "pl", "sh", "bat",
                "ps1", "sql", "xml", "html", "css", "scss", "sass", "less", "yaml", "yml", "toml",
                "ini", "cfg", "conf", "log",
            ],
        )
        .add_filter("All Files", &["*"])
        .save_file();

    match file_path {
        Some(path) => fs::write(&path, content)
            .await
            .map(|_| Some(path.to_string_lossy().to_string()))
            .map_err(|e| e.to_string()),
        None => Ok(None), // User cancelled the dialog
    }
}
