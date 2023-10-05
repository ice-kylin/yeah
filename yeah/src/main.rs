use std::fs;

use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use serde_yaml;
use tower_http::cors::CorsLayer;

#[derive(Deserialize, Serialize)]
struct Config {
    groups: Vec<Group>,
}

#[derive(Deserialize, Serialize)]
struct Group {
    name: String,
    items: Vec<Link>,
}

#[derive(Deserialize, Serialize)]
struct Link {
    name: String,
    logo: Option<Logo>,
    url: String,
    description: Option<String>,
    blank: Option<bool>,
    target: Option<String>,
}

#[derive(Deserialize, Serialize)]
enum Logo {
    Emj(String),
    Img(String),
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(links))
        .layer(CorsLayer::permissive());

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn get_links() -> Vec<Group> {
    let d: Config =
        serde_yaml::from_reader(fs::File::open("./config/config.yml").unwrap()).unwrap();

    d.groups
}

async fn links() -> Json<Vec<Group>> {
    Json(get_links())
}
