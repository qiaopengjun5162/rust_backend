use actix_web::{
    get, patch, post,
    web::{Data, Json, Path},
    App, HttpResponse, HttpServer, Responder,
};
use db::Database;
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::PizzaError,
    models::{pizza::Pizza, BuyPizzaRequest, UpdatePizzaURL},
};
mod db;
mod error;
mod models;

#[get("/pizzas")]
// async fn get_pizzas(db: Data<Database>) -> impl Responder {
async fn get_pizzas(db: Data<Database>) -> Result<Json<Vec<Pizza>>, PizzaError> {
    let pizzas = db.get_all_pizzas().await;
    match pizzas {
        // Some(found_pizzas) => HttpResponse::Ok().body(format!("{:#?}", found_pizzas)),
        Some(found_pizzas) => Ok(Json(found_pizzas)),
        None => Err(PizzaError::NoPizzasFound),
        // None => HttpResponse::Ok().body("No pizzas found!"),
    }
    // HttpResponse::Ok().body("Pizzas available!")
}

#[post("/buypizza")]
async fn buy_pizza(body: Json<BuyPizzaRequest>, db: Data<Database>) -> impl Responder {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let pizza_name = body.pizza_name.clone();
            let mut buffer = Uuid::encode_buffer();
            let new_uuid = Uuid::new_v4().simple().encode_lower(&mut buffer);

            let new_pizza = db
                .add_pizza(Pizza::new(String::from(new_uuid), pizza_name))
                .await;
            match new_pizza {
                Some(created) => {
                    HttpResponse::Ok().body(format!("Created new pizza: {:#?}", created))
                }
                None => HttpResponse::BadRequest().body("Could not create pizza!"),
            }
        }
        Err(e) => HttpResponse::BadRequest().body(format!("Pizza name required! Err: {}", e)),
    }
}

#[patch("/updatepizza/{uuid}")]
async fn update_pizza(update_pizza_url: Path<UpdatePizzaURL>) -> impl Responder {
    let uuid = update_pizza_url.into_inner().uuid;
    HttpResponse::Ok().body(format!("Updating the pizza with {uuid}!"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init()
        .await
        .expect("Failed to initialize database");
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(get_pizzas)
            .service(buy_pizza)
            .service(update_pizza)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
