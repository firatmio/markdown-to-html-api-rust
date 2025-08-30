use actix_files::Files;
use actix_web::{post, get, web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use serde::Deserialize;
use comrak::{markdown_to_html, ComrakOptions};

#[derive(Deserialize)]
struct RenderRequest {
    markdown: Option<String>,
    content: Option<String>,
}

#[post("/render")]
async fn render(req_body: web::Json<RenderRequest>) -> impl Responder {
    let md = req_body
        .markdown
        .as_ref()
        .or(req_body.content.as_ref())
        .map(|s| s.as_str())
        .unwrap_or("");

    let mut options = ComrakOptions::default();
    options.extension.table = true;
    options.extension.autolink = true;
    options.parse.smart = true;
    options.render.unsafe_ = true;

    let html = markdown_to_html(md, &options);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "status": "ok" }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    log::info!("🚀 Starting Markdown → HTML API server on http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(render)
            .service(health)
            .service(Files::new("/", "./frontend").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
