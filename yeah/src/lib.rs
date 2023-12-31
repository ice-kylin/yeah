use std::net::SocketAddr;
use std::sync::Arc;

use axum::{Router, Server};
use axum::routing::get;
use hyper::server::Builder;
use hyper::server::conn::AddrIncoming;
use log::{error, info};
use tower_http::cors;

use crate::config::AppConfig;
use crate::group::get_groups;
use crate::message::get_message;

mod common;
mod config;
mod group;
mod message;
mod note;
mod search;
mod service;
mod statistic;

struct YeahServer {
    builder: Builder<AddrIncoming>,
}

pub async fn run() {
    info!("🍻 Starting server...");

    let config = get_config();
    let app = Router::new()
        .route("/", get(get_groups))
        .route("/message", get(get_message))
        .with_state(config.clone())
        .layer(cors::CorsLayer::permissive());

    YeahServer::build(&config).serve(app).await;

    info!("👋 Bye!");
}

fn get_config() -> Arc<AppConfig> {
    let config = Arc::new(match AppConfig::new() {
        Ok(c) => c,
        Err(e) => {
            error!("Configuration file format error: {}", e);
            std::process::exit(exitcode::CONFIG);
        }
    });
    config
}

fn get_ip(config: &AppConfig) -> &str {
    match config.ip.as_ref() {
        None => "0.0.0.0:3000",
        Some(i) => i,
    }
}

fn parse_ip(ip: &str) -> SocketAddr {
    match ip.parse() {
        Ok(a) => a,
        Err(e) => {
            error!("IP address format error: {}", e);
            std::process::exit(exitcode::CONFIG);
        }
    }
}

impl YeahServer {
    fn build(config: &Arc<AppConfig>) -> Self {
        let ip = get_ip(config);

        let builder = match Server::try_bind(&parse_ip(ip)) {
            Ok(b) => {
                info!("🚀 Server running at {}", ip);
                b
            }
            Err(e) => {
                error!("Server error: {}", e);
                std::process::exit(exitcode::SOFTWARE);
            }
        };

        YeahServer { builder }
    }

    async fn serve(self, app: Router) {
        match self.builder.serve(app.into_make_service()).await {
            Ok(_) => {}
            Err(e) => {
                error!("Server error: {}", e);
                std::process::exit(exitcode::SOFTWARE);
            }
        }
    }
}
