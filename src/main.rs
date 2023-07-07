#![allow(unused)]

use crate::model::StringEntryController;

pub use self::error::{Error, Result};

use std::fmt::format;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use axum::extract::{Json, Query, State};
use axum::response::Response;
use axum::{middleware, routing::get, routing::post, Router};
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;

mod error;
mod model;
mod web;

#[derive(Clone)]
pub struct StringArr {
    data: Arc<Mutex<Vec<String>>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let model_controller = StringEntryController::new().await?;

    let app = Router::new()
        .nest(
            "/api",
            web::routes_string_arr::routes(model_controller.clone()),
        )
        .layer(middleware::map_response(response_mapper))
        .layer(CookieManagerLayer::new());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    print!("-->> LISTENING ON {addr}\n");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn response_mapper(res: Response) -> Response {
    println!("->> {:<12} - response mapper", "RES_MAPPER");

    println!();

    res
}
