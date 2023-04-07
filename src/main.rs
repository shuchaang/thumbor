use axum::{extract::Path, handler::get, http::StatusCode, Router};
use percent_encoding::percent_decode_str;use serde::Deserialize;use std::convert::TryInto;
mod pb;
use pb::*;

#[derive(Deserialize)]
struct Params{
    spec:String,
    url:String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/image/:spec/:url", get(generate));
    let addr = "127.0.0.1:3000".parse().unwrap();
    tracing::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn generate(Path(Params{spec,url}):Path<Params>) -> Result {
    let mut response = Response::new(StatusCode::OK);
    response.set_body(Body::from("Hello, world!"));
    response
}
