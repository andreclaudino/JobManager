use std::fmt::Display;

#[derive(Debug)]
pub(super) struct InternalError {
	error: anyhow::Error
}

impl Display for InternalError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{error}", error=self.error)
	}
}

impl actix_web::error::ResponseError for InternalError {
	fn status_code(&self) -> actix_web::http::StatusCode {
			actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
	}
}

impl From<anyhow::Error> for InternalError {
	fn from(error: anyhow::Error) -> InternalError {
		InternalError { error: error }
	}
}