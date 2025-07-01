use actix_web::{HttpRequest, HttpResponse, Responder};

pub fn render_user_response(user_content: String) -> String {
    format!("<div class='user-content'>{}</div>", user_content)
}

pub async fn display_content(req: HttpRequest) -> impl Responder {
    let content = req.match_info().get("content").unwrap_or("Welcome");
    HttpResponse::Ok().body(format!("<html><body><h1>User Content: {}</h1></body></html>", content)) //SINK
} 