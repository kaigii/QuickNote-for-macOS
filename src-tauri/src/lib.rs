// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod commands;
mod menu;
mod shortcuts;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .menu(menu::create_app_menu())
        .on_menu_event(|event| {
            menu::handle_menu_event(event.window(), event.menu_item_id());
        })
        .system_tray(menu::create_tray_menu())
        .on_system_tray_event(|app, event| {
            menu::handle_tray_event(app, event);
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            tauri::WindowEvent::FileDrop(file_drop_event) => {
                match file_drop_event {
                    tauri::FileDropEvent::Dropped(paths) => {
                        // Emit an event to the frontend with the file paths
                        event.window().emit("custom-file-drop", paths).unwrap();
                    }
                    _ => {
                        // Ignore Hovered and Cancelled events
                    }
                }
            }
            _ => {}
        })
        .setup(|app| {
            // Register default shortcuts on startup
            shortcuts::register_new_note_shortcut(&app.handle(), "CmdOrCtrl+Option+T");
            shortcuts::register_close_tab_shortcut(&app.handle(), "CmdOrCtrl+Option+Y");
            shortcuts::register_toggle_window_shortcut(&app.handle(), "CmdOrCtrl+Option+U");

            // Load settings and update shortcuts if different
            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move {
                if let Ok(settings) =
                    commands::settings_commands::load_settings(app_handle.clone()).await
                {
                    if settings.new_note_shortcut != "CmdOrCtrl+Option+T" {
                        shortcuts::register_new_note_shortcut(
                            &app_handle,
                            &settings.new_note_shortcut,
                        );
                    }
                    if settings.close_tab_shortcut != "CmdOrCtrl+Option+Y" {
                        shortcuts::register_close_tab_shortcut(
                            &app_handle,
                            &settings.close_tab_shortcut,
                        );
                    }
                }
            });

            #[cfg(target_os = "macos")]
            {
                use tauri::ActivationPolicy;
                app.set_activation_policy(ActivationPolicy::Accessory);
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::file_commands::open_file,
            commands::file_commands::save_file,
            commands::file_commands::save_file_as,
            commands::file_commands::open_specific_file,
            commands::settings_commands::load_settings,
            commands::settings_commands::save_settings,
            commands::settings_commands::select_directory,
            commands::settings_commands::update_shortcut,
            commands::settings_commands::update_tray_menu,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
