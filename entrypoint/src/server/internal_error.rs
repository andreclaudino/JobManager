use std::{fmt::Display, backtrace::Backtrace};

use actix_web::{HttpResponse, body::BoxBody, HttpResponseBuilder};
use serde::Serialize;

#[derive(Debug)]
pub(super) struct InternalError {
	pub(self) error: anyhow::Error
}

#[derive(Serialize)]
struct ErrorMessage {
	message: String,
	backtrace: String,
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

	fn error_response(&self) -> HttpResponse<BoxBody> {
		let error_message: ErrorMessage = self.get_error_message();
        let response =
			HttpResponseBuilder::new(self.status_code())
			.json(error_message);
		
		response
    }
}

impl From<anyhow::Error> for InternalError {
	fn from(error: anyhow::Error) -> InternalError {
		InternalError { error: error }
	}
}

impl InternalError {
	fn get_error_message(&self) -> ErrorMessage {
		let message = format!("{self}");
		let backtrace = format!("{}", Backtrace::capture());
		ErrorMessage { message, backtrace }
    }
}