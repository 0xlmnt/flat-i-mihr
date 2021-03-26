use uuid::Uuid;
use crate::security::crypt::{hash, generate_salt, salt_to_string};
use serde::{Serialize, Deserialize};
use argon2::verify_encoded;
use crate::schema::users;

#[derive(Queryable, Debug, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub firstname: String,
    pub lastname: Option<String>,
    pub password: String,
    pub salt: String,
    pub email: String,
    pub wk_key: Option<String>,
    pub settings: Option<String>
}
impl User {
    pub fn new(uname: String, email: String, password: String) -> Self {
        let salt = generate_salt();
        let hash = hash(password, &salt);

        User {
            id: Uuid::new_v4(),
            username: uname,
            firstname: "".to_string(),
            lastname: None,
            password: hash,
            salt: salt_to_string(&salt),
            email,
            wk_key: None,
            settings: None
        }
    }

    pub fn from_insertable(insertable: InsertableUser) -> Self {
        User::new(insertable.username, insertable.email, insertable.password)
    }

    pub fn match_pwd(&self, password: String) -> bool {
        verify_encoded(&self.password, password.as_bytes()).unwrap()
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertableUser {
    pub username: String,
    pub email: String,
    pub password: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseUser {
    pub id: String,
    pub username: String,
    pub email: String
}
impl ResponseUser {
    pub fn from_user(user: &User) -> Self {
        ResponseUser {
            id: user.id.to_string(),
            username: String::from(&user.username),
            email: String::from(&user.email)
        }
    }
}

pub struct UserPassword {
    pub password: String,
    pub new_password: Option<String>
}
