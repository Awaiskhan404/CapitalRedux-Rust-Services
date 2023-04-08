use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::errors::ApiError;
use crate::models::{AppState, User};
use crate::services::{JwtService, UserService};

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct CreateUserResponse {
    token: String,
}

pub async fn create_user(
    app_state: web::Data<AppState>,
    user_data: web::Json<CreateUserRequest>,
) -> Result<impl Responder, ApiError> {
    let user_service = UserService::new(&app_state.pool);
    let jwt_service = JwtService::new(&app_state.jwt_secret)?;

    let user = user_service.create_user(&user_data.email, &user_data.password)?;
    let token = jwt_service.generate_token(user.id)?;

    Ok(HttpResponse::Ok().json(CreateUserResponse { token }))
}