use axum::{extract::State, routing::get, Json, Router};
use serde_json::{json, Value};

use crate::{Error, Result, StringArr};

pub fn routes(state: StringArr) -> Router {
    Router::new().route("/", get(get_handler)).with_state(state)
}

async fn get_handler(state: State<StringArr>) -> Result<Json<Value>> {
    let mut shared_state = state.data.lock().unwrap();

    if shared_state.is_empty() {
        return Err(Error::EmptyArr);
    }

    let body = Json(json!({
      "result": *shared_state
    }));

    Ok(body)
}
