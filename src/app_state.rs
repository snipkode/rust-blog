use sea_orm::DatabaseConnection;
use tera::Tera;
use axum_extra::extract::cookie::Key;

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
    pub templates: Tera,
    pub key: Key,
}
