#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use c3s::Entry;
use postgres::{Client, NoTls};

#[put("/")]
fn create_entry() -> &'static str {
    "Criou entrada"
}

#[get("/valor?<value>")]
fn get_by_value(value: Option<i32>) -> String {
    // Abre a conexao com o banco de dados
    let mut conn = Client::connect("host=localhost dbname=erp-database user=locutor", NoTls)
        .expect("Failed to connect to database.");

    let lancamentos = Entry::get_by(&mut conn, "valor", value.unwrap());

    // Retorna os lancamentos em formato de DEBUG ainda
    format!("{:?}", lancamentos)
}

#[get("/dia?<value>")]
fn get_by_day(value: Option<i32>) -> String {
    // abre a conexao com o banco de dados
    let mut conn = Client::connect("host=localhost dbname=erp-database user=locutor", NoTls)
        .expect("failed to connect to database.");

    let lancamentos = Entry::get_by(&mut conn, "dia", value.unwrap());

    // retorna os lancamentos em formato de debug ainda
    format!("{:?}", lancamentos)
}

#[get("/origem?<value>")]
fn get_by_origin(value: Option<i32>) -> String {
    // abre a conexao com o banco de dados
    let mut conn = Client::connect("host=localhost dbname=erp-database user=locutor", NoTls)
        .expect("failed to connect to database.");

    let lancamentos = Entry::get_by(&mut conn, "origem", value.unwrap());

    // retorna os lancamentos em formato de debug ainda
    format!("{:?}", lancamentos)
}

#[get("/destino?<value>")]
fn get_by_destination(value: Option<i32>) -> String {
    // abre a conexao com o banco de dados
    let mut conn = Client::connect("host=localhost dbname=erp-database user=locutor", NoTls)
        .expect("failed to connect to database.");

    let lancamentos = Entry::get_by(&mut conn, "destino", value.unwrap());

    // retorna os lancamentos em formato de debug ainda
    format!("{:?}", lancamentos)
}

#[put("/delete")]
fn delete_entry() -> &'static str {
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
            routes![
                create_entry,
                get_by_value,
                get_by_day,
                get_by_origin,
                get_by_destination,
                delete_entry
            ],
        )
        .mount("/relatorio", routes![get_report])
        .launch();
}
