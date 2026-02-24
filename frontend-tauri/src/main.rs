use careless::audio::AudioCaptureSession;
use careless::config::ConfigStore;
use std::sync::Mutex;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{Emitter, Manager};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_global_shortcut::Builder as GlobalShortcutBuilder;

fn build_tauri_builder() -> tauri::Builder<tauri::Wry> {
    let config_store = ConfigStore::new(config_path());
    let capture_session = Mutex::new(AudioCaptureSession::default());

    tauri::Builder::default()
        .manage(config_store)
        .manage(capture_session)
        .invoke_handler(tauri::generate_handler![
            careless::tauri_commands::app_health_command,
            careless::tauri_commands::system_capability_command,
            careless::tauri_commands::validate_model_path_command,
            careless::tauri_commands::validate_audio_path_command,
            careless::tauri_commands::pick_model_path_command,
            careless::tauri_commands::pick_audio_path_command,
            careless::tauri_commands::run_transcription_command,
            careless::tauri_commands::run_transcription_with_options_command,
            careless::tauri_commands::export_transcript_command,
            careless::tauri_commands::open_output_folder_command,
            careless::tauri_commands::register_global_shortcut_command,
            careless::tauri_commands::unregister_global_shortcut_command,
            careless::tauri_commands::is_shortcut_registered_command,
            careless::tauri_commands::get_app_settings_command,
            careless::tauri_commands::set_start_in_background_command,
            careless::tauri_commands::set_launch_on_login_command,
            careless::tauri_commands::list_presets_command,
            careless::tauri_commands::get_preset_command,
            careless::tauri_commands::save_preset_command,
            careless::tauri_commands::delete_preset_command,
            careless::tauri_commands::set_default_preset_command,
            careless::tauri_commands::get_default_preset_command,
            careless::tauri_commands::add_to_queue_command,
            careless::tauri_commands::remove_from_queue_command,
            careless::tauri_commands::reorder_queue_command,
            careless::tauri_commands::get_queue_command,
            careless::tauri_commands::clear_completed_queue_command,
            careless::tauri_commands::update_queue_item_status_command,
            careless::tauri_commands::add_to_history_command,
            careless::tauri_commands::get_history_command,
            careless::tauri_commands::clear_history_command,
            careless::tauri_commands::copy_to_clipboard_command,
            careless::tauri_commands::get_clipboard_text_command,
            careless::tauri_commands::get_ptt_routing_command,
            careless::tauri_commands::set_ptt_routing_command,
            careless::tauri_commands::set_theme_command,
            careless::tauri_commands::set_default_output_dir_command,
            careless::tauri_commands::set_default_model_dir_command,
            careless::tauri_commands::set_diagnostics_enabled_command,
            careless::tauri_commands::set_default_threads_command,
            careless::tauri_commands::set_default_timeout_command,
            careless::tauri_commands::pick_directory_command,
            careless::tauri_commands::list_audio_devices_command,
            careless::tauri_commands::select_audio_device_command,
            careless::tauri_commands::get_capture_state_command,
            careless::tauri_commands::start_capture_command,
            careless::tauri_commands::stop_capture_command,
            careless::tauri_commands::get_current_audio_device_command,
        ])
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(
            GlobalShortcutBuilder::new()
                .with_handler(|app, shortcut, event| {
                    use tauri_plugin_global_shortcut::ShortcutState;

                    println!(
                        "careless: global shortcut event - {:?} ({:?})",
                        shortcut,
                        event.state()
                    );

                    // Handle PTT shortcut (configured as "ptt" in UI)
                    // For now, we emit an event that the frontend can handle
                    if let Some(window) = app.get_webview_window("main") {
                        let event_name = match event.state() {
                            ShortcutState::Pressed => "ptt-start",
                            ShortcutState::Released => "ptt-stop",
                        };
                        let _ = window.emit(event_name, ());
                    }
                })
                .build(),
        )
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--hidden"]),
        ))
        .setup(|app| {
            let config_store = app.state::<ConfigStore>();
            let config = config_store.load().unwrap_or_default();
            let start_in_background = config.start_in_background;

            let open_item = MenuItem::with_id(app, "open", "Open App", true, None::<&str>)?;
            let hide_item = MenuItem::with_id(app, "hide", "Hide App", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

            let tray_menu = Menu::with_items(app, &[&open_item, &hide_item, &quit_item])?;

            TrayIconBuilder::new()
                .menu(&tray_menu)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "open" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.unminimize();
                            let _ = window.set_focus();
                        }
                    }
                    "hide" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.hide();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            match window.is_visible() {
                                Ok(true) => {
                                    let _ = window.hide();
                                }
                                Ok(false) | Err(_) => {
                                    let _ = window.show();
                                    let _ = window.unminimize();
                                    let _ = window.set_focus();
                                }
                            }
                        }
                    }
                })
                .build(app)?;

            // Hide window on startup if start_in_background is enabled
            if start_in_background && let Some(window) = app.get_webview_window("main") {
                let _ = window.hide();
            }

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
}

fn config_path() -> std::path::PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    path.push("careless");
    path.push("config.json");
    path
}

fn main() {
    build_tauri_builder()
        .run(tauri::generate_context!())
        .expect("failed to run careless");
}

#[cfg(test)]
mod tests {
    use super::build_tauri_builder;

    #[test]
    fn tauri_builder_function_is_wired() {
        let _builder_fn: fn() -> tauri::Builder<tauri::Wry> = build_tauri_builder;
    }
}
