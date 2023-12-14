// data trait for pizza
use crate::{db::Database, models::pizza::Pizza};
use actix_web::web::Data;
use async_trait::async_trait;
use surrealdb::Error;

#[async_trait]
pub trait PizzaDataTrait {
    // get all pizzas from the database
    async fn get_all_pizzas(db: &Data<Database>) -> Option<Vec<Pizza>>;
    // add a new pizza to the database
    async fn add_pizza(db: &Data<Database>, new_pizza: Pizza) -> Option<Pizza>;
    // update an existing pizza in the database
    async fn update_pizza(db: &Data<Database>, uuid: String) -> Option<Pizza>;
}

#[async_trait]
impl PizzaDataTrait for Database {
    // get all pizzas from the database
    async fn get_all_pizzas(db: &Data<Database>) -> Option<Vec<Pizza>> {
        // query the database for all pizzas
        let result = db.client.select("pizza").await;
        // if the query was successful, return the pizzas
        match result {
            Ok(all_pizzas) => Some(all_pizzas),
            // otherwise, return nothing
            Err(_) => None,
        }
    }

    // add a new pizza to the database
    async fn add_pizza(db: &Data<Database>, new_pizza: Pizza) -> Option<Pizza> {
        // create a new pizza in the database
        let created_pizza = db
            .client
            .create(("pizza", new_pizza.uuid.clone()))
            .content(new_pizza)
            .await;

        // if the creation was successful, return the pizza
        match created_pizza {
            Ok(created_pizza) => created_pizza,
            // otherwise, return nothing
            Err(_) => None,
        }
    }

    // update an existing pizza in the database
    async fn update_pizza(db: &Data<Database>, uuid: String) -> Option<Pizza> {
        // query the database for the pizza
        let find_pizza: Result<Option<Pizza>, Error> = db.client.select(("pizza", &uuid)).await;
        // if the query was successful, return the pizza
        match find_pizza {
            Ok(found) => {
                match found {
                    // if the pizza was found
                    Some(_found_pizza) => {
                        // and if found the pizza
                        let updated_pizza: Result<Option<Pizza>, Error> = db
                            .client
                            .update(("pizza", &uuid))
                            .merge(Pizza {
                                uuid,
                                pizza_name: String::from("sold"),
                            })
                            .await;
                        // if the update was successful, return the pizza
                        match updated_pizza {
                            Ok(updated_pizza) => updated_pizza,
                            // otherwise, return nothing
                            Err(_) => None,
                        }
                    }
                    // if the pizza was not found, return nothing
                    None => None,
                }
            }
            // if the query was unsuccessful, return nothing
            Err(_) => None,
        }
    }
}
