#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
extern crate argon2;

mod routes;
mod models;
mod schema;
mod security;

use self::diesel::prelude::*;
use self::models::*;

#[database("postgres_db")]
pub struct Database(diesel::PgConnection);

#[get("/")]
fn index() -> String {
    "We're online".to_string()
}

#[get("/daddy")]
fn daddy() -> String {
    String::from("Yes papi it's me and Ive got issues")
}


fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        //.attach(Database::fairing())
        .launch();
}
