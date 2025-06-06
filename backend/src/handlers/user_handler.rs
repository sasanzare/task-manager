use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use uuid::Uuid;
use crate::models::user::{NewUser, LoginUser};
use crate::models::user::repository::UserRepository;

pub async fn register(
    pool: web::Data<sqlx::PgPool>,
    new_user: web::Json<NewUser>,
) -> impl Responder {
    match UserRepository::create(&pool, new_user.into_inner()).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => {
            HttpResponse::BadRequest().json(json!({"error": e.to_string()}))
        }
    }
}

pub async fn login(
    pool: web::Data<sqlx::PgPool>,
    credentials: web::Json<LoginUser>,
) -> impl Responder {
    match UserRepository::verify_credentials(&pool, credentials.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(json!({
            "message": "Login successful",
            "user": {
                "id": user.id,
                "username": user.username,
                "email": user.email
            }
        })),
        Err(e) => HttpResponse::Unauthorized().json(json!({"error": e})),
    }
}

pub async fn get_user(
    pool: web::Data<sqlx::PgPool>,
    user_id: web::Path<Uuid>,
) -> impl Responder {
    match UserRepository::find_by_id(&pool, user_id.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::NotFound().json(json!({"error": e.to_string()})),
    }
}