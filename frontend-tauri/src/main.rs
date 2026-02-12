fn build_tauri_builder() -> tauri::Builder<tauri::Wry> {
    tauri::Builder::default().invoke_handler(tauri::generate_handler![
        frontend_tauri::tauri_commands::app_health_command,
        frontend_tauri::tauri_commands::system_capability_command,
        frontend_tauri::tauri_commands::validate_model_path_command,
        frontend_tauri::tauri_commands::validate_audio_path_command,
        frontend_tauri::tauri_commands::run_transcription_command,
        frontend_tauri::tauri_commands::run_transcription_with_options_command,
    ])
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
