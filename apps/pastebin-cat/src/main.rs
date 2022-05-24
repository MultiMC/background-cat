use axum::{
    extract::Path,
    http::{HeaderValue, StatusCode},
    response::{Html, IntoResponse},
    routing::get,
};
use std::net::SocketAddr;
use tera::Tera;

#[tokio::main]
async fn main() {
    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        ("paste.html", include_str!("paste.html")),
        ("error.html", include_str!("error.html")),
    ]).expect("Failed to add templates to templating library!");
    let http_client = reqwest::Client::builder()
        .referer(false)
        .user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION")
        ))
        .build()
        .expect("Failed to create HTTP client");
    let app = axum::Router::new().route(
        "/:channelid/:messageid/:filename",
        get(move |path| get_file(path, http_client, tera)),
    );
    let listen = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("[INFO] Listening on http://{}", &listen);
    axum::Server::bind(&listen)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start the server");
}

async fn get_file(
    Path((channelid, messageid, filename)): Path<(String, String, String)>,
    http: reqwest::Client,
    tera: tera::Tera,
) -> Result<impl IntoResponse, Error> {
    let req = http
        .get(format!(
            "https://cdn.discordapp.com/attachments/{}/{}/{}",
            channelid, messageid, filename
        ))
        .build()?;
    let resp = http.execute(req).await?;
    let headers = resp.headers();
    if !headers
        .get("Content-Type")
        .unwrap_or(&HeaderValue::from_static(""))
        .to_str()?
        .to_string()
        .to_ascii_lowercase()
        .contains("charset=utf-8")
    {
        return Err(Error::NotFound);
    }
    let data = resp.text().await?;
    let mut ctx = tera::Context::new();
    ctx.insert("paste", &data);
    Ok(Html(tera.render("paste.html", &ctx)?))
}

enum Error {
    NotFound,
    Reqwest(reqwest::Error),
    Templating(tera::Error),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(_: std::string::FromUtf8Error) -> Self {
        Self::NotFound
    }
}

impl From<reqwest::header::ToStrError> for Error {
    fn from(_: reqwest::header::ToStrError) -> Self {
        Self::NotFound
    }
}

impl From<tera::Error> for Error {
    fn from(e: tera::Error) -> Self {
        Self::Templating(e)
    }
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (error, status): (String, StatusCode) = match self {
            Error::NotFound => ("404 paste not found".to_string(), StatusCode::NOT_FOUND),
            Error::Reqwest(e) => (
                format!("Discord returned an error: {:?}", e),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            Error::Templating(e) => (
                format!("Templating library returned an error: {:?}", e),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
        };
        let mut ctx = tera::Context::new();
        ctx.insert("error", &error);
        let err_html = Tera::one_off(include_str!("error.html"), &ctx, true)
            .unwrap_or_else(|_| include_str!("templating_error.html").to_string());
        axum::response::Response::builder()
            .status(status)
            .body(axum::body::boxed(axum::body::Full::from(err_html)))
            .unwrap()
    }
}
