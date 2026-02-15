use tauri::Manager;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri_plugin_global_shortcut::Builder as GlobalShortcutBuilder;

fn build_tauri_builder() -> tauri::Builder<tauri::Wry> {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            frontend_tauri::tauri_commands::app_health_command,
            frontend_tauri::tauri_commands::system_capability_command,
            frontend_tauri::tauri_commands::validate_model_path_command,
            frontend_tauri::tauri_commands::validate_audio_path_command,
            frontend_tauri::tauri_commands::pick_model_path_command,
            frontend_tauri::tauri_commands::pick_audio_path_command,
            frontend_tauri::tauri_commands::run_transcription_command,
            frontend_tauri::tauri_commands::run_transcription_with_options_command,
            frontend_tauri::tauri_commands::export_transcript_command,
            frontend_tauri::tauri_commands::open_output_folder_command,
            frontend_tauri::tauri_commands::register_global_shortcut_command,
            frontend_tauri::tauri_commands::unregister_global_shortcut_command,
        ])
        .plugin(
            GlobalShortcutBuilder::new()
                .with_handler(|_app, shortcut, event| {
                    println!(
                        "frontend-tauri: global shortcut event - {:?} ({:?})",
                        shortcut,
                        event.state()
                    );
                })
                .build(),
        )
        .setup(|app| {
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

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
}

fn main() {
    build_tauri_builder()
        .run(tauri::generate_context!())
        .expect("failed to run frontend-tauri");
}

#[cfg(test)]
mod tests {
    use super::build_tauri_builder;

    #[test]
    fn tauri_builder_function_is_wired() {
        let _builder_fn: fn() -> tauri::Builder<tauri::Wry> = build_tauri_builder;
    }
}
