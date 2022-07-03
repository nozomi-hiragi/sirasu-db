use std::env;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use async_graphql::{InputObject, SimpleObject};
use mongodb::bson::{doc, from_bson};
use pbkdf2::Pbkdf2;
use scrypt::Scrypt;
use serde::{Deserialize, Serialize};

use crate::utils::{gen_random_string, jwt::make_jwt, mongodb::get_client};

#[derive(SimpleObject, Clone, Serialize, Deserialize)]
pub struct User {
    #[graphql(name = "_id")]
    pub _id: String,
    pub pass: String,
    pub allowed_collections: Vec<String>,
}

fn get_sirasu_config_db() -> mongodb::sync::Database {
    let client = get_client().unwrap();
    client.database("sirasu_config")
}

#[derive(InputObject)]
pub struct SirasuSigninInput {
    id: String,
    pass: String,
    key: String,
}
pub async fn sirasu_signin(input: SirasuSigninInput) -> Result<String, String> {
    let signin_pass = env::var("SIGNIN_PASS").unwrap_or("pass".to_string());
    if input.key != signin_pass {
        return Err(format!("wrong key"));
    }

    let salt = gen_random_string();
    let pass_hash =
        match PasswordHash::generate(Argon2::default(), input.pass.as_str(), salt.as_str()) {
            Ok(hash) => Ok(hash),
            Err(err) => Err(err.to_string()),
        }?;

    let user = User {
        _id: input.id,
        pass: pass_hash.to_string(),
        allowed_collections: vec![],
    };

    let db = get_sirasu_config_db();
    let user_collection = db.collection("user");
    match user_collection.count_documents(doc! {"_id":&user._id}, None) {
        Ok(count) => {
            if count <= 0 {
                Ok(())
            } else {
                Err("Already exist this id")
            }
        }
        Err(_) => Ok(()),
    }?;

    let ret = match user_collection.insert_one(user.clone(), None) {
        Ok(v) => Ok(v),
        Err(err) => Err(format!("insert error: {}", err.to_string())),
    }?;

    match from_bson::<String>(ret.inserted_id) {
        Ok(v) => Ok(v),
        Err(err) => Err(format!("bson error: {}", err.to_string())),
    }
}

#[derive(InputObject)]
pub struct SirasuLoginInput {
    id: String,
    pass: String,
}
pub async fn sirasu_login(input: SirasuLoginInput) -> Result<String, String> {
    let id = input.id.as_str();
    let db = get_sirasu_config_db();
    let user_collection = db.collection::<User>("user");
    let user = match user_collection.find_one(doc! { "_id":id }, None) {
        Ok(user) => match user {
            Some(user) => Ok(user),
            None => Err(format!("Uset not found")),
        },
        Err(err) => Err(format!("find error: {}", err.to_string())),
    }?;

    let password_hash = PasswordHash::new(user.pass.as_str()).unwrap();
    let algs: &[&dyn PasswordVerifier] = &[&Argon2::default(), &Pbkdf2, &Scrypt];
    match password_hash.verify_password(algs, input.pass.as_bytes()) {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("pass varify error: {}", err.to_string())),
    }?;

    let token = make_jwt(id);

    Ok(format!("{}", token))
}
