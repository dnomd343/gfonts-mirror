mod upstream;

use actix_web::error::{Error, ErrorInternalServerError, ErrorServiceUnavailable};
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, get, http, web};
use http::header::{ACCESS_CONTROL_ALLOW_ORIGIN, USER_AGENT};

#[get("/css")]
async fn css_entry(req: HttpRequest) -> Result<impl Responder, Error> {
    let query = req.query_string();
    let user_agent = match req.headers().get(USER_AGENT) {
        Some(val) => val.to_str().unwrap(),
        None => "Default UA",
    };

    match upstream::css_fetch(query, user_agent).await {
        Ok(content) => {
            let resp = HttpResponse::Ok()
                .content_type("text/css")
                .append_header((ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
                .body(content);
            Ok(resp)
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            Err(ErrorInternalServerError(err))
        }
    }
}

#[get("/fonts/{path:.*}")]
async fn font_entry(path: web::Path<String>) -> Result<impl Responder, Error> {
    match upstream::font_fetch(&path).await {
        Ok((mime, content)) => {
            let resp = HttpResponse::Ok()
                .content_type(mime)
                .append_header((ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
                .body(content);
            Ok(resp)
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            Err(ErrorServiceUnavailable(err))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(css_entry).service(font_entry))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
