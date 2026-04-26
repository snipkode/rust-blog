use axum::{
    routing::{get, post},
    Router,
};
use axum_extra::extract::cookie::Key;
use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use rust_blog::{app_state::AppState, handlers::post as post_handlers, handlers::auth as auth_handlers};
use sea_orm::{ConnectOptions, Database};
use std::env;
use std::time::Duration;
use tera::Tera;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let cookie_secret = env::var("COOKIE_SECRET").expect("COOKIE_SECRET must be set");

    // Connect to database with options
    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(10)
       .min_connections(2)
       .connect_timeout(Duration::from_secs(10))
       .acquire_timeout(Duration::from_secs(10))
       .idle_timeout(Duration::from_secs(10))
       .max_lifetime(Duration::from_secs(10))
       .sqlx_logging(true);

    let conn = Database::connect(opt).await.expect("Failed to connect to database");

    // Run migrations
    Migrator::up(&conn, None).await.expect("Failed to run migrations");

    // Initialize templates
    let templates = Tera::new("templates/**/*").expect("Failed to initialize templates");

    let key = Key::from(cookie_secret.as_bytes());

    let state = AppState { conn, templates, key };

    // Set up routes
    let app = Router::new()
        .route("/", get(post_handlers::list_posts))
        .route("/post/new", get(post_handlers::new_post_form).post(post_handlers::create_post))
        .route("/post/{id}", get(post_handlers::view_post))
        .route("/post/{id}/edit", get(post_handlers::edit_post_form).post(post_handlers::update_post))
        .route("/post/{id}/delete", post(post_handlers::delete_post))
        .route("/register", get(auth_handlers::register_form).post(auth_handlers::register))
        .route("/login", get(auth_handlers::login_form).post(auth_handlers::login))
        .route("/logout", get(auth_handlers::logout))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    let addr = format!("{}:{}", host, port);
    println!("Listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
