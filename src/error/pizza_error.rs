// will contain our new enum pizza error here

use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum PizzaError {
    /// No pizzas found
    NoPizzasFound = 0,
    /// Pizza creation failed
    PizzaCreationFailure = 1,
    /// No such pizza found
    NoSuchPizzaFound = 2,
}

impl ResponseError for PizzaError {
    /// Create an HTTP response with the correct status code and body
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    /// Return the status code for this error
    fn status_code(&self) -> StatusCode {
        match self {
            PizzaError::NoPizzasFound => StatusCode::NOT_FOUND,
            PizzaError::PizzaCreationFailure => StatusCode::INTERNAL_SERVER_ERROR,
            PizzaError::NoSuchPizzaFound => StatusCode::NOT_FOUND,
        }
    }
}
