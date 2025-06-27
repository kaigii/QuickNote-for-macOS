use std::collections::HashMap;
use std::sync::Mutex;
use tauri::{
    AppHandle, CustomMenuItem, Manager, Menu, MenuItem, Submenu, SystemTray, SystemTrayEvent,
    SystemTrayMenu, SystemTrayMenuItem, SystemTraySubmenu, Window,
};

// Global state to store recent files for each app instance
lazy_static::lazy_static! {
    static ref RECENT_FILES: Mutex<HashMap<String, Vec<String>>> = Mutex::new(HashMap::new());
}

/// Creates the main application menu
pub fn create_app_menu() -> Menu {
    // File menu
    let file_menu = Submenu::new(
        "File",
        Menu::new()
            .add_item(CustomMenuItem::new("new_note", "New Note").accelerator("CmdOrCtrl+N"))
            .add_item(CustomMenuItem::new("open_file", "Open...").accelerator("CmdOrCtrl+O"))
            .add_item(CustomMenuItem::new("save_file", "Save").accelerator("CmdOrCtrl+S"))
            .add_item(
                CustomMenuItem::new("save_file_as", "Save As...").accelerator("CmdOrCtrl+Shift+S"),
            )
            .add_item(CustomMenuItem::new("close_tab", "Close Tab").accelerator("CmdOrCtrl+W"))
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Quit),
    );

    // Edit menu with standard editing operations
    let edit_menu = Submenu::new(
        "Edit",
        Menu::new()
            .add_native_item(MenuItem::Undo)
            .add_native_item(MenuItem::Redo)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Cut)
            .add_native_item(MenuItem::Copy)
            .add_native_item(MenuItem::Paste)
            .add_native_item(MenuItem::SelectAll),
    );

    // View menu
    let view_menu = Submenu::new(
        "View",
        Menu::new().add_item(CustomMenuItem::new("toggle_theme", "Toggle Theme")),
    );

    // Main menu
    Menu::new()
        .add_submenu(file_menu)
        .add_submenu(edit_menu)
        .add_submenu(view_menu)
}

/// Handles main menu events by emitting them to the frontend
pub fn handle_menu_event(window: &Window, menu_id: &str) {
    match menu_id {
        "new_note" | "open_file" | "save_file" | "save_file_as" | "toggle_theme" | "close_tab" => {
            // Emit the menu event to the frontend
            if let Err(e) = window.emit("menu-event", menu_id) {
                eprintln!("Failed to emit menu event: {}", e);
            }
        }
        _ => {}
    }
}

// --- System Tray ---

/// Creates the system tray menu
pub fn create_tray_menu() -> SystemTray {
    let new_note = CustomMenuItem::new("tray_new_note".to_string(), "New Note");
    let close_tab = CustomMenuItem::new("tray_close_tab".to_string(), "Close Current Tab");
    let open_file = CustomMenuItem::new("tray_open_file".to_string(), "Open...");
    let save_file = CustomMenuItem::new("tray_save".to_string(), "Save");
    let save_file_as = CustomMenuItem::new("tray_save_as".to_string(), "Save As...");
    let show_hide = CustomMenuItem::new("tray_show_hide".to_string(), "Show/Hide Window");
    let quit = CustomMenuItem::new("tray_quit".to_string(), "Quit");

    // Create recent files submenu (placeholder for now)
    let recent_files_submenu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("tray_no_recent".to_string(), "No Recent Files").disabled());

    let tray_menu = SystemTrayMenu::new()
        .add_item(new_note)
        .add_item(close_tab)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(open_file)
        .add_item(save_file)
        .add_item(save_file_as)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_submenu(SystemTraySubmenu::new(
            "Recently Closed",
            recent_files_submenu,
        ))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(show_hide)
        .add_item(quit);

    SystemTray::new().with_menu(tray_menu)
}

/// Updates the system tray menu with recent files
pub fn update_tray_menu_with_recent_files(app: &AppHandle, recent_files: Vec<String>) {
    // Store recent files in global state
    let app_id = app.config().tauri.bundle.identifier.clone();
    {
        let mut files = RECENT_FILES.lock().unwrap();
        files.insert(app_id, recent_files.clone());
    }

    let new_note = CustomMenuItem::new("tray_new_note".to_string(), "New Note");
    let close_tab = CustomMenuItem::new("tray_close_tab".to_string(), "Close Current Tab");
    let open_file = CustomMenuItem::new("tray_open_file".to_string(), "Open...");
    let save_file = CustomMenuItem::new("tray_save".to_string(), "Save");
    let save_file_as = CustomMenuItem::new("tray_save_as".to_string(), "Save As...");
    let show_hide = CustomMenuItem::new("tray_show_hide".to_string(), "Show/Hide Window");
    let quit = CustomMenuItem::new("tray_quit".to_string(), "Quit");

    // Create recent files submenu
    let mut recent_files_submenu = SystemTrayMenu::new();
    if recent_files.is_empty() {
        recent_files_submenu = recent_files_submenu.add_item(
            CustomMenuItem::new("tray_no_recent".to_string(), "No Recent Files").disabled(),
        );
    } else {
        for (index, file_path) in recent_files.iter().enumerate() {
            let file_name = std::path::Path::new(file_path)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("Unknown File");

            let menu_id = format!("tray_recent_{}", index);
            recent_files_submenu =
                recent_files_submenu.add_item(CustomMenuItem::new(menu_id, file_name));
        }
    }

    let tray_menu = SystemTrayMenu::new()
        .add_item(new_note)
        .add_item(close_tab)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(open_file)
        .add_item(save_file)
        .add_item(save_file_as)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_submenu(SystemTraySubmenu::new(
            "Recently Closed",
            recent_files_submenu,
        ))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(show_hide)
        .add_item(quit);

    // Update the system tray menu
    let system_tray = app.tray_handle();
    let _ = system_tray.set_menu(tray_menu);
}

/// Handles system tray events
pub fn handle_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    if let Some(window) = app.get_window("main") {
        match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                match id.as_str() {
                    "tray_show_hide" => toggle_window_visibility(&window),
                    "tray_new_note" => window.emit("menu-event", "new_note").unwrap(),
                    "tray_open_file" => window.emit("menu-event", "open_file").unwrap(),
                    "tray_save" => window.emit("menu-event", "save_file").unwrap(),
                    "tray_save_as" => window.emit("menu-event", "save_file_as").unwrap(),
                    "tray_close_tab" => window.emit("menu-event", "close_active_tab").unwrap(),
                    "tray_no_recent" => {
                        // No action for disabled placeholder item
                    }
                    "tray_quit" => app.exit(0),
                    _ => {
                        // Handle recent file clicks
                        if id.starts_with("tray_recent_") {
                            if let Some(index_str) = id.strip_prefix("tray_recent_") {
                                if let Ok(index) = index_str.parse::<usize>() {
                                    // Get the recent files from the global state
                                    let app_id = app.config().tauri.bundle.identifier.clone();
                                    if let Ok(files) = RECENT_FILES.lock() {
                                        if let Some(recent_files) = files.get(&app_id) {
                                            if index < recent_files.len() {
                                                let file_path = &recent_files[index];
                                                let _ = window.emit("open-recent-file", file_path);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            // Left click on tray icon - do nothing
            SystemTrayEvent::LeftClick { .. } => {
                // No action - tray icon click is disabled
            }
            _ => {}
        }
    }
}

fn toggle_window_visibility(window: &Window) {
    if window.is_visible().unwrap_or(false) {
        window.hide().unwrap();
    } else {
        window.show().unwrap();
        window.set_focus().unwrap();
    }
}
