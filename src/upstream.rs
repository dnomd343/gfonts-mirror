use bytes::Bytes;
use reqwest::ClientBuilder;
use reqwest::header::CONTENT_TYPE;
use std::time::Duration;

const API: &str = "https://cdn.dnomd343.top/fonts/";

pub async fn css_fetch(params: &str, ua: &str) -> Result<String, reqwest::Error> {
    let url = format!("https://fonts.googleapis.com/css?{}", params);
    eprintln!("CSS: {} [{}]", url, ua);

    let client = ClientBuilder::new()
        .user_agent(ua)
        .timeout(Duration::from_secs(15))
        .build()?;

    let resp = client.get(url).send().await?.error_for_status()?;
    let content = resp.text().await?;
    Ok(content.replace("https://fonts.gstatic.com/", API))
}

pub async fn font_fetch(path: &str) -> Result<(String, Bytes), reqwest::Error> {
    let url = format!("https://fonts.gstatic.com/{}", path);
    eprintln!("Font: {}", url);

    let client = ClientBuilder::new()
        .timeout(Duration::from_secs(15))
        .build()?;
    let resp = client.get(url).send().await?.error_for_status()?;

    let mime = String::from(match resp.headers().get(CONTENT_TYPE) {
        Some(value) => value.to_str().unwrap(),
        None => "application/octet-stream",
    });
    Ok((mime, resp.bytes().await?))
}
