use crate::models::user::{ResponseUser, User, InsertableUser};
use uuid::Uuid;
use rocket_contrib::databases::diesel::PgConnection;
use crate::schema::users::dsl::users;
use rocket::http::RawStr;
use rocket_contrib::json::Json;
use diesel::RunQueryDsl;
use crate::Database;

#[get("/users")]
pub fn get_users() -> String {
    "".to_string()
}

#[post("/users", format = "json", data = "<user>")]
pub fn create_user(c: Database, user: Json<InsertableUser>) {
    let to_insert = User::from_insertable(user.0);

    diesel::insert_into(users)
        .values(to_insert)
        .get_result::<User>(&*c)
        .expect("Error on inserting User");

}