#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod config;
mod hotkey;

use commands::AppState;
use std::sync::Mutex;
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::TrayIconBuilder;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState {
            has_open_window: Mutex::new(false),
        })
        .setup(|app| {
            let handle = app.handle().clone();

            // Load hotkey config
            let cfg = config::load();

            // Build tray menu
            let about_item = MenuItemBuilder::with_id("about", "关于 QRCode Tool").build(app)?;
            let quit_item = MenuItemBuilder::with_id("quit", "退出").build(app)?;
            let menu = MenuBuilder::new(app)
                .item(&about_item)
                .item(&quit_item)
                .build()?;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "quit" => app.exit(0),
                    "about" => {
                        // TODO: show about dialog
                    }
                    _ => {}
                })
                .build(app)?;

            // Start global keyboard hook
            hotkey::start(handle, cfg);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::handle_trigger,
            commands::notify_closed,
        ])
        .run(tauri::generate_context!())
        .expect("error running qrcode-tool");
}
