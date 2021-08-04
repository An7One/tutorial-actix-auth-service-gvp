mod auth;
mod user;

use auth::auth;
use actix_web::{web, HttpResponse};
use user::{create_user, me, update_profile};

use crate::error::AppError;

type AppResult<T> = Result<T, AppError>;
type AppResponse = AppResult<HttpResponse>;

pub fn app_config(config: &mut web::ServiceConfig){
    let signUp = web::resource("/signup").route(web::post().to(create_user));

    let auth = web::resource("/auth").route(web::post().to(auth));

    let me = web::resource("/me")
        .route(web::get().to(me))
        .route(web::post().to(update_profile));

    config.service(signUp).service(auth).service(me);
}
