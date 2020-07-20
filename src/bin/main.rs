#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use erp::{Entry, NewEntry};
use postgres::{Client, NoTls};
use rocket::http::RawStr;
use rocket_contrib::{json::Json, uuid::Uuid as RocketUuid};
use uuid::Uuid;

#[post("/", format = "json", data = "<entry>")]
fn create_entry(entry: Json<NewEntry>) -> &'static str {
    let new_entry = entry.into_inner();

    if !new_entry.is_valid() {
        panic!()
    }

    // Abre a conexao com o banco de dados
    let mut conn = Client::connect("host=localhost dbname=erp-database user=locutor", NoTls)
        .expect("Failed to connect to database.");
    let mut tr = conn.transaction().expect("Failed to initiate transaction");

    new_entry
        .persist(&mut tr)
        .expect("Failed to persist to database.");

    "Criou entrada"
}

#[get("/?<param>&<value>")]
fn get_entries(param: &RawStr, value: i32) -> String {
    // Abre a conexão com o banco de dados
    let mut conn = Client::connect("host=localhost dbname=erp-database user=locutor", NoTls)
        .expect("failed to connect to database.");

    let entries = Entry::get_by(&mut conn, param.as_str(), value).expect("Failed to get entries.");

    // Retorna os lançamentos em formato de debug ainda
    let mut s = String::new();
    for entry in entries {
        s.insert_str(0, &format!("{:?}\n", entry));
    }
    return s;
}

#[put("/delete?<id>")]
fn delete_entry(id: RocketUuid) -> &'static str {
    // Abre a conexao com o banco de dados
    let mut conn = Client::connect("host=localhost dbname=erp-database user=locutor", NoTls)
        .expect("Failed to connect to database.");
    let mut tr = conn.transaction().expect("Failed to initiate transaction");

    let id = Uuid::from_bytes(*id.as_bytes());

    Entry::delete(&mut tr, id).expect("Failed to delete");

    "Deletou entrada"
}

#[get("/")]
fn get_report() -> &'static str {
    "Baixou relatorio"
}

fn main() {
    rocket::ignite()
        .mount(
            "/lancamento",
            routes![create_entry, get_entries, delete_entry],
        )
        .mount("/relatorio", routes![get_report])
        .launch();
}
