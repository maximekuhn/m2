use derive_new::new;

#[derive(new, Clone)]
pub struct AppState {
    pub db_pool: sqlx::SqlitePool,
}
