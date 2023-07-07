use axum::{extract::State, routing::post, Json, Router};
use tower_cookies::{Cookie, Cookies};

use crate::{RequestData, StringArr};

pub fn routes(state: StringArr) -> Router {
    Router::new()
        .route("/", post(set_handler))
        .with_state(state)
}

async fn set_handler(
    cookies: Cookies,
    State(state): State<StringArr>,
    Json(data): Json<RequestData>,
) -> String {
    cookies.add(Cookie::new("auth-token", "test"));
    let mut shared_state = state.data.lock().unwrap();
    shared_state.push(data.value);
    format!("Data: {:?}", *shared_state)
}
