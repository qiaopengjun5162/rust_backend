// will contain our new enum pizza error here

use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum PizzaError {
    NoPizzasFound = 0,
    PizzaCreationFailure = 1,
    NoSuchPizzaFound = 2,
}

impl ResponseError for PizzaError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            PizzaError::NoPizzasFound => StatusCode::NOT_FOUND,
            PizzaError::PizzaCreationFailure => StatusCode::INTERNAL_SERVER_ERROR,
            PizzaError::NoSuchPizzaFound => StatusCode::NOT_FOUND,
        }
    }
}