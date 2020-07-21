use crate::err::Result;

use postgres::{Client, NoTls};
use std::env;

pub fn get_conn() -> Result<Client> {
    let instance = env::var("INSTANCE_CONNECTION_NAME").unwrap();

    Ok(Client::configure()
        .user(&env::var("DB_USER").unwrap())
        .password(&env::var("DB_PASS").unwrap())
        .dbname(&env::var("DB_NAME").unwrap())
        .host(&format!("/cloudsql/{}", instance))
        .port(5432)
        .connect(NoTls)?)
}
