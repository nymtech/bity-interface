use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::log::error;

#[derive(Debug)]
pub struct AppError(pub StatusCode, pub Option<String>);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        self.1
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error> + std::fmt::Debug,
{
    fn from(err: E) -> Self {
        error!("{:#?}", err);
        AppError(StatusCode::INTERNAL_SERVER_ERROR, None)
    }
}
