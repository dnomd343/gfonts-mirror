use std::time::Duration;
use tokio;
use reqwest::{Client, ClientBuilder};
use bytes::Bytes;

use http::header::{USER_AGENT, CONTENT_TYPE, ACCESS_CONTROL_ALLOW_ORIGIN};

use axum::{
    routing::{get, post},
    http::StatusCode,
    Router,
    extract::RawQuery,
    http::HeaderMap,
    response::Response,
    extract::Path
};
use axum::response::IntoResponse;
use tokio::net::TcpListener;

const API: &str = "https://cdn.dnomd343.top/fonts/";

async fn css_fetch(params: &str, ua: &str) -> Result<String, reqwest::Error> {
    let url = format!("https://fonts.googleapis.com/css?{}", params);
    eprintln!("CSS: {} [{}]", url, ua);

    let client = ClientBuilder::new()
        .user_agent(ua)
        .timeout(Duration::from_secs(15))
        .build()?;

    let resp = client.get(url)
        .send().await?
        .error_for_status()?;
    let content = resp.text().await?;
    Ok(content.replace("https://fonts.gstatic.com/", API))
}

async fn font_fetch(path: &str) -> Result<(String, Bytes), reqwest::Error> {
    let url = format!("https://fonts.gstatic.com/{}", path);
    eprintln!("Font: {}", url);

    let client = ClientBuilder::new()
        .timeout(Duration::from_secs(15))
        .build()?;
    let resp = client.get(url)
        .send().await?
        .error_for_status()?;

    let mime = String::from(match resp.headers().get(CONTENT_TYPE) {
        Some(value) => value.to_str().unwrap(),
        None => "application/octet-stream",
    });
    Ok((mime, resp.bytes().await?))
}

async fn css_entry(RawQuery(query): RawQuery, headers: HeaderMap) -> impl IntoResponse {
    let query = query.unwrap_or(String::new());
    let ua = match headers.get(USER_AGENT) {
        Some(val) => val.to_str().unwrap(),
        None => "Default UA",
    };
    let headers = [
        (CONTENT_TYPE, "text/css"),
        (ACCESS_CONTROL_ALLOW_ORIGIN, "*")
    ];
    match css_fetch(&query, ua).await {
        Ok(content) => (StatusCode::OK, headers, content),
        Err(e) => {
            eprintln!("Error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, headers, e.to_string())
        }
    }
}

async fn font_entry(Path(path): Path<String>) -> Response {
    // eprintln!("Query: {}", path);
    match font_fetch(&path).await {
        Ok((mime, content)) => {
            let headers = [
                (ACCESS_CONTROL_ALLOW_ORIGIN, "*"),
                (CONTENT_TYPE, &mime)
            ];
            (StatusCode::OK, headers, content).into_response()
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            StatusCode::SERVICE_UNAVAILABLE.into_response()
        }
    }
}

#[tokio::main]
async fn main() {
    // let ret = css_fetch("family=Roboto", "UA").await;
    // println!("{:?}", ret);

    // let ret = font_fetch("s/roboto/v47/KFOMCnqEu92Fr1ME7kSn66aGLdTylUAMQXC89YmC2DPNWubEbVmUiA8.ttf").await;
    // println!("{:?}", ret);

    let app = Router::new()
        .route("/css", get(css_entry))
        .route("/fonts/{*path}", get(font_entry));
    let listener = TcpListener::bind("0.0.0.0:47114").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
