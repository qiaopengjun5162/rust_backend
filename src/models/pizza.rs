// pizza.rs
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct BuyPizzaRequest {
    #[validate(length(min = 1, message = "pizza name is required"))]
    pub pizza_name: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdatePizzaURL {
    pub uuid: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Pizza {
    pub uuid: String,
    pub pizza_name: String,
}

impl Pizza {
    pub fn new(uuid: String, pizza_name: String) -> Self {
        Self { uuid, pizza_name }
    }
}
