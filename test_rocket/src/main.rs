use rocket::form::Form;
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;

#[derive(FromForm, Serialize, Deserialize)]
struct UserForm{
    name: String,
    age: u8,
}