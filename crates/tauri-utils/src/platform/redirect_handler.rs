use axum::response::Redirect;

pub async fn execute_redirect(target_url: String) -> Redirect {
    //SINK
    Redirect::to(&target_url)
}

pub async fn redirect_endpoint(target_url: String) -> Redirect {
    execute_redirect(target_url).await
} 