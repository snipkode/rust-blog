use axum::{
    extract::{State, FromRequestParts},
    response::{Html, IntoResponse, Redirect},
    Form,
    http::request::Parts,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use sea_orm::{ActiveModelTrait, EntityTrait, Set, ColumnTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use crate::app_state::AppState;
use crate::entities::{user, User};
use tera::Context;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use std::env;

#[derive(Deserialize)]
pub struct AuthForm {
    pub username: String,
    pub password: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub struct UserContext(pub Option<Claims>);

impl FromRequestParts<AppState> for Claims {
    type Rejection = Redirect;

    async fn from_request_parts(parts: &mut Parts, _state: &AppState) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_headers(&parts.headers);

        let token = jar.get("jwt")
            .map(|c| c.value().to_string())
            .ok_or_else(|| Redirect::to("/login"))?;

        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        decode::<Claims>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|_| Redirect::to("/login"))
    }
}

impl FromRequestParts<AppState> for UserContext {
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &AppState) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_headers(&parts.headers);

        let token = match jar.get("jwt") {
            Some(c) => c.value().to_string(),
            None => return Ok(UserContext(None)),
        };

        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        let claims = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .ok();

        Ok(UserContext(claims))
    }
}

pub async fn register_form(State(state): State<AppState>) -> impl IntoResponse {
    let rendered = state.templates.render("register.html", &Context::new()).unwrap();
    Html(rendered)
}

pub async fn register(
    State(state): State<AppState>,
    Form(form): Form<AuthForm>,
) -> impl IntoResponse {
    let password_hash = hash(form.password, DEFAULT_COST).unwrap();

    let user = user::ActiveModel {
        username: Set(form.username),
        password_hash: Set(password_hash),
        ..Default::default()
    };

    match user.insert(&state.conn).await {
        Ok(_) => Redirect::to("/login"),
        Err(_) => Redirect::to("/register"), // Should handle error better in real app
    }
}

pub async fn login_form(State(state): State<AppState>) -> impl IntoResponse {
    let rendered = state.templates.render("login.html", &Context::new()).unwrap();
    Html(rendered)
}

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Form(form): Form<AuthForm>,
) -> impl IntoResponse {
    let user = User::find()
        .filter(user::Column::Username.eq(form.username.clone()))
        .one(&state.conn)
        .await
        .unwrap();

    if let Some(user) = user {
        if verify(form.password, &user.password_hash).unwrap() {
            let expiration = chrono::Utc::now()
                .checked_add_signed(chrono::Duration::hours(24))
                .expect("valid timestamp")
                .timestamp() as usize;

            let claims = Claims {
                sub: user.username,
                exp: expiration,
            };

            let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
            let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap();

            let cookie = Cookie::build(("jwt", token))
                .path("/")
                .http_only(true)
                .build();

            return (jar.add(cookie), Redirect::to("/")).into_response();
        }
    }

    Redirect::to("/login").into_response()
}

pub async fn logout(jar: CookieJar) -> impl IntoResponse {
    (jar.remove(Cookie::from("jwt")), Redirect::to("/")).into_response()
}
