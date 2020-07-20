#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use erp::{
    err::{ErpError, Result},
    Entry, NewEntry,
};
use postgres::{Client, NoTls};
use rocket::http::RawStr;
use rocket_contrib::{json::Json, uuid::Uuid as RocketUuid};
use uuid::Uuid;

#[post("/", format = "json", data = "<entry>")]
fn create_entry(entry: Json<NewEntry>) -> Result<()> {
    let new_entry = entry.into_inner();

    // Se a entrada é inválida, retorna BadRequest.
    if !new_entry.is_valid() {
        return Err(ErpError::BadRequest);
    }

    // Abre a conexão com o banco de dados.
    let mut conn = Client::connect("host=localhost dbname=erp-database user=locutor", NoTls)?;
    let mut tr = conn.transaction()?;

    // Persiste o lançamento no banco de dados.
    new_entry.persist(&mut tr)?;

    Ok(())
}

#[get("/?<param>&<value>")]
fn get_entries(param: &RawStr, value: i32) -> Result<Json<Vec<Entry>>> {
    // Abre a conexão com o banco de dados
    let mut conn = Client::connect("host=localhost dbname=erp-database user=locutor", NoTls)?;

    // Busca os lançamentos com base no parâmetro informado.
    let entries = Entry::get_by(&mut conn, param.as_str(), value)?;

    // Retorna os lançamentos em formato JSON.
    Ok(Json(entries))
}

#[put("/delete?<id>")]
fn delete_entry(id: RocketUuid) -> Result<()> {
    // Abre a conexão com o banco de dados.
    let mut conn = Client::connect("host=localhost dbname=erp-database user=locutor", NoTls)?;
    let mut tr = conn.transaction()?;

    // Devido a versões conflitantes de Uuid, é necessária a conversão entre o
    // RocketUuid e o Uuid aqui.
    let id = Uuid::from_bytes(*id.as_bytes());

    // Deleta o lançamento com o id informado do banco de dados.
    //
    // No futuro, é melhor utilizar deleção lógica ao invés de absoluta.
    Entry::delete(&mut tr, id)?;

    Ok(())
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
