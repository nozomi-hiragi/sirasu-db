use std::sync::Mutex;

use mongodb::{error::Error, options::ClientOptions, sync::Client};
use once_cell::sync::Lazy;

use crate::utils::params::Params;

pub fn connect_to_db(url: &str) -> Result<Client, Error> {
    let client_options = ClientOptions::parse(url)?;
    let client = Client::with_options(client_options)?;
    return Ok(client);
}

pub fn get_client() -> Result<Client, Error> {
    static URL_MUTEX: Lazy<Mutex<String>> = Lazy::new(|| {
        let params = Params::new();
        let urls = params.mongodb_url.clone();
        Mutex::new(urls)
    });
    let url_string = URL_MUTEX.lock().unwrap();
    connect_to_db(url_string.as_str())
}
