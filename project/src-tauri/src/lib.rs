mod commands;
mod db;
mod error;
mod models;
mod utils;

use db::Database;
use std::sync::Arc;
use tauri::{
    menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem},
    tray::{TrayIconBuilder, TrayIconEvent},
    Manager, WebviewUrl, WebviewWindowBuilder,
};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tracing_subscriber::{fmt, EnvFilter};

pub struct AppState {
    pub database: Arc<Database>,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse().unwrap()))
        .init();

    tracing::info!("Application starting");

    tracing::info!(
        "Log de erros em {}",
        utils::error_log::log_file_path().display()
    );

    // Registra panics no ficheiro de log de erros mantendo o comportamento padrão.
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        utils::error_log::log_panic(info);
        default_hook(info);
    }));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .setup(|app| {
            let home = std::env::var("USERPROFILE").expect("failed to get USERPROFILE env var");
            let app_data_dir = std::path::PathBuf::from(home).join(".ghpm");
            std::fs::create_dir_all(&app_data_dir).expect("failed to create .ghpm directory");

            let db_path = app_data_dir.join("app_data.sqlite");
            let database = Database::new(&db_path).expect("failed to initialize database");
            database
                .init_and_migrate()
                .expect("failed to run migrations");

            tracing::info!("Database initialized at {}", db_path.display());

            app.manage(AppState {
                database: Arc::new(database),
            });

            // Setup tray icon
            let show_item = MenuItem::with_id(app, "show_main", "Abrir GHPM", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Sair", true, None::<&str>)?;

            // Auto-start check item
            let autostart_manager = app.autolaunch();
            let is_autostart_enabled = autostart_manager.is_enabled().unwrap_or(false);
            let autostart_item = CheckMenuItem::with_id(
                app,
                "autostart",
                "Iniciar com o Windows",
                true,
                is_autostart_enabled,
                None::<&str>,
            )?;
            let separator1 = PredefinedMenuItem::separator(app)?;
            let separator2 = PredefinedMenuItem::separator(app)?;

            let menu = Menu::with_items(
                app,
                &[&show_item, &separator1, &autostart_item, &separator2, &quit_item],
            )?;

            let _tray = TrayIconBuilder::with_id("main-tray")
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("GHPM - GitHub Profile Manager")
                .menu(&menu)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "show_main" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "autostart" => {
                        let autostart_manager = app.autolaunch();
                        if autostart_manager.is_enabled().unwrap_or(false) {
                            let _ = autostart_manager.disable();
                            tracing::info!("Auto-start disabled");
                        } else {
                            let _ = autostart_manager.enable();
                            tracing::info!("Auto-start enabled");
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: tauri::tray::MouseButton::Left,
                        button_state: tauri::tray::MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        // Check if tray-popup already exists
                        if let Some(popup) = app.get_webview_window("tray-popup") {
                            if popup.is_visible().unwrap_or(false) {
                                let _ = popup.hide();
                            } else {
                                let _ = popup.show();
                                let _ = popup.set_focus();
                            }
                        } else {
                            // Get monitor size for positioning
                            let (screen_width, screen_height) =
                                if let Some(monitor) = app.primary_monitor().unwrap() {
                                    let size = monitor.size();
                                    (size.width as f64, size.height as f64)
                                } else {
                                    (1920.0, 1080.0)
                                };

                            let popup_width = 250.0;
                            let popup_height = 40.0;
                            let x = screen_width - popup_width - 16.0;
                            let y = screen_height - popup_height - 48.0;

                            let _ = WebviewWindowBuilder::new(
                                app,
                                "tray-popup",
                                WebviewUrl::App("index.html#tray".into()),
                            )
                            .title("GHPM")
                            .inner_size(popup_width, popup_height)
                            .position(x, y)
                            .decorations(false)
                            .resizable(false)
                            .always_on_top(true)
                            .skip_taskbar(true)
                            .build();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                // For main window: hide instead of close (go to tray)
                if window.label() == "main" {
                    window.hide().ok();
                    api.prevent_close();
                }
                // For tray-popup: just hide
                if window.label() == "tray-popup" {
                    window.hide().ok();
                    api.prevent_close();
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::profile_commands::get_all_profiles,
            commands::profile_commands::get_profile_by_id,
            commands::profile_commands::create_profile,
            commands::profile_commands::update_profile,
            commands::profile_commands::delete_profile,
            commands::profile_commands::set_active_profile,
            commands::ssh_commands::list_ssh_keys,
            commands::ssh_commands::generate_ssh_key,
            commands::ssh_commands::delete_ssh_key,
            commands::ssh_commands::test_ssh_connection,
            commands::ssh_commands::read_ssh_key_content,
            commands::git_commands::get_global_git_identity,
            commands::git_commands::set_global_git_identity,
            commands::settings_commands::get_settings,
            commands::settings_commands::update_settings,
            commands::settings_commands::get_activity_logs,
            commands::credential_commands::save_pat,
            commands::credential_commands::get_pat,
            commands::credential_commands::delete_pat,
            commands::log_commands::log_frontend_error,
            commands::backup_commands::get_backup_info,
            commands::backup_commands::export_backup,
            commands::backup_commands::import_backup,
            commands::repository_commands::list_repositories,
            commands::repository_commands::list_scan_roots,
            commands::repository_commands::add_scan_root,
            commands::repository_commands::remove_scan_root,
            commands::repository_commands::scan_repositories,
            commands::repository_commands::apply_profile_to_repository,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
