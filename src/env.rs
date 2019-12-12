pub fn init() {
    std::env::set_var("APP_HOST", "localhost:8080");
    std::env::set_var("UI_HOST", "localhost:8000");
    std::env::set_var("DB_HOST", "jookbachs.db");
}
