use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse, Redirect},
    Form,
};
use sea_orm::{ActiveModelTrait, EntityTrait, Set, QueryOrder, Order};
use serde::Deserialize;
use crate::app_state::AppState;
use crate::entities::{post, Post};
use tera::Context;

#[derive(Deserialize)]
pub struct CreatePost {
    pub title: String,
    pub content: String,
}

use crate::handlers::auth::{Claims, UserContext};

pub async fn list_posts(
    UserContext(user): UserContext,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let posts = Post::find()
        .order_by(post::Column::CreatedAt, Order::Desc)
        .all(&state.conn)
        .await
        .unwrap();

    let mut ctx = Context::new();
    ctx.insert("posts", &posts);
    ctx.insert("user", &user);

    let rendered = state.templates.render("index.html", &ctx).unwrap();
    Html(rendered)
}

pub async fn view_post(
    UserContext(user): UserContext,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let post = Post::find_by_id(id).one(&state.conn).await.unwrap().unwrap();

    let mut ctx = Context::new();
    ctx.insert("post", &post);
    ctx.insert("user", &user);

    let rendered = state.templates.render("post.html", &ctx).unwrap();
    Html(rendered)
}

pub async fn new_post_form(
    _claims: Claims,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let mut ctx = Context::new();
    ctx.insert("user", &_claims);
    
    let rendered = state.templates.render("new.html", &ctx).unwrap();
    Html(rendered)
}

pub async fn create_post(
    _claims: Claims,
    State(state): State<AppState>,
    Form(form): Form<CreatePost>,
) -> impl IntoResponse {
    let post = post::ActiveModel {
        title: Set(form.title),
        content: Set(form.content),
        ..Default::default()
    };

    post.insert(&state.conn).await.unwrap();

    Redirect::to("/")
}

pub async fn edit_post_form(
    _claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let post = Post::find_by_id(id).one(&state.conn).await.unwrap().unwrap();

    let mut ctx = Context::new();
    ctx.insert("post", &post);
    ctx.insert("user", &_claims);

    let rendered = state.templates.render("edit.html", &ctx).unwrap();
    Html(rendered)
}

pub async fn update_post(
    _claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Form(form): Form<CreatePost>,
) -> impl IntoResponse {
    let post = Post::find_by_id(id).one(&state.conn).await.unwrap().unwrap();
    let mut post: post::ActiveModel = post.into();

    post.title = Set(form.title);
    post.content = Set(form.content);
    post.updated_at = Set(chrono::Utc::now().naive_utc());

    post.update(&state.conn).await.unwrap();

    Redirect::to(&format!("/post/{}", id))
}

pub async fn delete_post(
    _claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    Post::delete_by_id(id).exec(&state.conn).await.unwrap();

    Redirect::to("/")
}
