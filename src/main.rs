#![allow(unused)]

use crate::model::StringEntryController;

pub use self::error::{Error, Result};

use std::fmt::format;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use axum::extract::{Json, Query, State};
use axum::response::{IntoResponse, Response};
use axum::{middleware, routing::get, routing::post, Router};
use serde_json::json;
use tower_cookies::CookieManagerLayer;
use uuid::Uuid;

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

    let uuid = Uuid::new_v4();

    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!(
                {
                    "error": {
                        "type" : client_error.as_ref(),
                        "req_uuid": uuid.to_string()
                    }
                }
            );
            println!("-->> client_error_body: {client_error_body}");
            (*status_code, Json(client_error_body)).into_response()
        });

    println!(" -->> server log line - {uuid} - Error: {service_error:?}");

    println!();

    error_response.unwrap_or(res)
}
