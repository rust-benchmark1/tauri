use actix_web::{web, HttpResponse, Responder};
use actix_web::redirect::Redirect;

pub async fn execute_redirect(target_url: String) -> impl Responder {
    //SINK
    Redirect::to(&target_url)
}

pub async fn redirect_endpoint(target_url: String) -> impl Responder {
    execute_redirect(target_url).await
} 