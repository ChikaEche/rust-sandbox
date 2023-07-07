use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    EmptyArr,
    EntryNotFound { id: u64 },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        print!("-->> {:<12} - {self:?}", "INTO_RES");

        (StatusCode::INTERNAL_SERVER_ERROR, "CLIENT_ERROR").into_response()
    }
}
