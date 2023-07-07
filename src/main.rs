#![allow(unused)]

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
mod web;

#[derive(Clone)]
pub struct StringArr {
    data: Arc<Mutex<Vec<String>>>,
}

#[derive(Deserialize)]
struct RequestData {
    value: String,
}

#[tokio::main]
async fn main() {
    let state = StringArr {
        data: Arc::new(Mutex::new(vec![])),
    };

    let app = Router::new()
        .merge(web::routes_get_arr::routes(state.clone()))
        .merge(web::routes_set_arr::routes(state.clone()))
        .layer(middleware::map_response(response_mapper))
        .layer(CookieManagerLayer::new());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    print!("-->> LISTENING ON {addr}\n");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn response_mapper(res: Response) -> Response {
    println!("->> {:<12} - response mapper", "RES_MAPPER");

    println!();

    res
}
