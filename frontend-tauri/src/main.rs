fn app_health() -> String {
    frontend_tauri::commands::app_health().status
}

fn main() {
    println!("frontend-tauri: {}", app_health());
}

#[cfg(test)]
mod tests {
    use super::app_health;

    #[test]
    fn app_health_is_ok() {
        assert_eq!(app_health(), "ok".to_owned());
    }
}
