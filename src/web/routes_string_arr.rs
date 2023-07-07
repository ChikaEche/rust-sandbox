use axum::{
    extract::{FromRef, Path, State},
    routing::{delete, get, post},
    Json, Router,
};

use crate::{model::RequestData, model::StringEntry, model::StringEntryController, Result};

#[derive(Clone, FromRef)]
struct AppState {
    controller: StringEntryController,
}

pub fn routes(controller: StringEntryController) -> Router {
    let app_state = AppState { controller };
    Router::new()
        .route("/", get(list_entry).post(create_entry))
        .route("/:id", delete(delete_entry))
        .with_state(app_state)
}

async fn create_entry(
    State(controller): State<StringEntryController>,
    Json(string_entry): Json<RequestData>,
) -> Result<Json<StringEntry>> {
    println!("->> {:<12} - create entry", "HANDLER");

    let entry = controller.create_entry(string_entry).await?;
    Ok(Json(entry))
}

async fn list_entry(
    State(controller): State<StringEntryController>,
) -> Result<Json<Vec<StringEntry>>> {
    println!("->> {:<12} - list entry", "HANDLER");

    let entries = controller.list_entry().await?;
    Ok(Json(entries))
}

async fn delete_entry(
    Path(id): Path<u64>,
    State(controller): State<StringEntryController>,
) -> Result<Json<StringEntry>> {
    println!("->> {:<12} - delete entry", "HANDLER");
    let entry = controller.delete_entry(id).await?;
    Ok(Json(entry))
}
