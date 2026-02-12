fn app_health() -> &'static str {
    "ok"
}

fn main() {
    println!("frontend-tauri: {}", app_health());
}

#[cfg(test)]
mod tests {
    use super::app_health;

    #[test]
    fn app_health_is_ok() {
        assert_eq!(app_health(), "ok");
    }
}
