use crate::err::Result;

use postgres::{Client, NoTls};
use std::env;

pub fn get_conn() -> Result<Client> {
    Ok(Client::configure()
        .user(&env::var("DB_USER").unwrap())
        .password(&env::var("DB_PASS").unwrap())
        .host(&env::var("DB_HOST").unwrap())
        .dbname(&env::var("DB_NAME").unwrap())
        .connect(NoTls)?)
}
