use std::thread;
use std::time::Duration;
use tauri::GlobalShortcutManager;
use tauri::{AppHandle, Manager};

const NEW_NOTE_SHORTCUT_ID: &str = "new_note_shortcut";
const CLOSE_TAB_SHORTCUT_ID: &str = "close_tab_shortcut";
const TOGGLE_WINDOW_SHORTCUT_ID: &str = "toggle_window_shortcut";

pub fn register_new_note_shortcut(app: &AppHandle, shortcut_str: &str) {
    let mut shortcut_manager = app.global_shortcut_manager();

    // Unregister the old shortcut first
    let _ = shortcut_manager.unregister(NEW_NOTE_SHORTCUT_ID);

    let main_window = match app.get_window("main") {
        Some(w) => w,
        None => {
            eprintln!("Main window not found, skipping shortcut registration");
            return;
        }
    };

    let shortcut = shortcut_str.to_string();
    match shortcut_manager.register(&shortcut, move || {
        let _ = main_window.hide();
        let _ = main_window.show();
        let _ = main_window.set_focus();
        let _ = main_window.emit("shortcut-new-note", ());
        let _ = main_window.set_always_on_top(true);
        thread::spawn({
            let main_window = main_window.clone();
            move || {
                thread::sleep(Duration::from_millis(1000));
                let _ = main_window.set_always_on_top(false);
            }
        });
    }) {
        Ok(_) => println!("Successfully registered shortcut: {}", shortcut),
        Err(e) => eprintln!("Failed to register shortcut '{}': {:?}", shortcut, e),
    }
}

pub fn register_close_tab_shortcut(app: &AppHandle, shortcut_str: &str) {
    let mut shortcut_manager = app.global_shortcut_manager();
    let _ = shortcut_manager.unregister(CLOSE_TAB_SHORTCUT_ID);
    let main_window = match app.get_window("main") {
        Some(w) => w,
        None => {
            eprintln!("Main window not found, skipping shortcut registration");
            return;
        }
    };
    let shortcut = shortcut_str.to_string();
    match shortcut_manager.register(&shortcut, move || {
        let _ = main_window.hide();
        let _ = main_window.show();
        let _ = main_window.set_focus();
        let _ = main_window.emit("shortcut-close-tab", ());
        let _ = main_window.set_always_on_top(true);
        thread::spawn({
            let main_window = main_window.clone();
            move || {
                thread::sleep(Duration::from_millis(1000));
                let _ = main_window.set_always_on_top(false);
            }
        });
    }) {
        Ok(_) => println!("Successfully registered close tab shortcut: {}", shortcut),
        Err(e) => eprintln!(
            "Failed to register close tab shortcut '{}': {:?}",
            shortcut, e
        ),
    }
}

pub fn register_toggle_window_shortcut(app: &AppHandle, shortcut_str: &str) {
    let mut shortcut_manager = app.global_shortcut_manager();
    let _ = shortcut_manager.unregister(TOGGLE_WINDOW_SHORTCUT_ID);
    let main_window = match app.get_window("main") {
        Some(w) => w,
        None => {
            eprintln!("Main window not found, skipping shortcut registration");
            return;
        }
    };
    let shortcut = shortcut_str.to_string();
    match shortcut_manager.register(&shortcut, move || {
        if main_window.is_visible().unwrap_or(false) {
            let _ = main_window.hide();
        } else {
            let _ = main_window.show();
            let _ = main_window.set_focus();
        }
        let _ = main_window.set_always_on_top(true);
        thread::spawn({
            let main_window = main_window.clone();
            move || {
                thread::sleep(Duration::from_millis(1000));
                let _ = main_window.set_always_on_top(false);
            }
        });
    }) {
        Ok(_) => println!(
            "Successfully registered toggle window shortcut: {}",
            shortcut
        ),
        Err(e) => eprintln!(
            "Failed to register toggle window shortcut '{}': {:?}",
            shortcut, e
        ),
    }
}
