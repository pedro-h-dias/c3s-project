#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use erp::{
    database::get_conn,
    err::{ErpError, Result},
    Entry, NewEntry, Report,
};
use rocket_contrib::{json::Json, uuid::Uuid as RocketUuid};
use uuid::Uuid;

#[get("/")]
fn help() -> String {
    r#"
        USAGE:

        GET /lancamento/                Retorna todos lançamentos do último mês.
        GET /lancamento/?valor=         Retorna os lançamentos com um dado valor (float32).
        GET /lancamento/?dia=           Retorna os lançamentos em um dado dia (entre 1 e 30).
        GET /lancamento/?origem=        Retorna os lançamentos com uma dada origem (entre 1 e 10).
        GET /lancamento/?destino=       Retorna os lançamentos com um dado destino (entre 1 e 10).
        PUT /lancamento/delete?id=      Deleta um lançamento dado seu identificador UUID.
        POST /lancamento/               Insere um lançamento novo.

        O input para um novo lançamento é esperado no formato JSON, conforme o exemplo a seguir.
        {
           "valor": 13.37,
           "dia": 25,
           "class": "Receita",
           "origem": 2
        }
        "#
    .to_owned()
}

#[post("/", format = "json", data = "<entry>")]
fn create_entry(entry: Json<NewEntry>) -> Result<()> {
    let new_entry = entry.into_inner();

    // Se a entrada é inválida, retorna BadRequest.
    if !new_entry.is_valid() {
        return Err(ErpError::BadRequest);
    }

    // Abre a conexão com o banco de dados.
    let mut conn = get_conn()?;
    let mut tr = conn.transaction()?;

    // Persiste o lançamento no banco de dados.
    new_entry.persist(&mut tr)?;
    tr.commit()?;

    Ok(())
}

#[get("/")]
fn get_all_entries() -> Result<Json<Vec<Entry>>> {
    // Abre a conexão com o banco de dados
    let mut conn = get_conn()?;

    // Busca os lançamentos com base no parâmetro informado.
    let entries = Entry::get_all(&mut conn)?;

    // Retorna os lançamentos em formato JSON.
    Ok(Json(entries))
}

#[get("/valor?<value>")]
fn get_entries_by_value(value: f32) -> Result<Json<Vec<Entry>>> {
    // Abre a conexão com o banco de dados
    let mut conn = get_conn()?;

    // Busca os lançamentos com base no parâmetro informado.
    let entries = Entry::get_by(&mut conn, "valor", None, Some(value))?;

    // Retorna os lançamentos em formato JSON.
    Ok(Json(entries))
}

#[get("/dia?<value>")]
fn get_entries_by_day(value: i32) -> Result<Json<Vec<Entry>>> {
    // Abre a conexão com o banco de dados
    let mut conn = get_conn()?;

    // Busca os lançamentos com base no parâmetro informado.
    let entries = Entry::get_by(&mut conn, "dia", Some(value), None)?;

    // Retorna os lançamentos em formato JSON.
    Ok(Json(entries))
}

#[get("/origem?<value>")]
fn get_entries_by_origin(value: i32) -> Result<Json<Vec<Entry>>> {
    // Abre a conexão com o banco de dados
    let mut conn = get_conn()?;

    // Busca os lançamentos com base no parâmetro informado.
    let entries = Entry::get_by(&mut conn, "origem", Some(value), None)?;

    // Retorna os lançamentos em formato JSON.
    Ok(Json(entries))
}

#[get("/destino?<value>")]
fn get_entries_by_destination(value: i32) -> Result<Json<Vec<Entry>>> {
    // Abre a conexão com o banco de dados
    let mut conn = get_conn()?;

    // Busca os lançamentos com base no parâmetro informado.
    let entries = Entry::get_by(&mut conn, "destino", Some(value), None)?;

    // Retorna os lançamentos em formato JSON.
    Ok(Json(entries))
}

#[put("/delete?<id>")]
fn delete_entry(id: RocketUuid) -> Result<()> {
    // Abre a conexão com o banco de dados.
    let mut conn = get_conn()?;
    let mut tr = conn.transaction()?;

    // Devido a versões conflitantes de Uuid, é necessária a conversão entre o
    // RocketUuid e o Uuid aqui.
    let id = Uuid::from_bytes(*id.as_bytes());

    // Deleta o lançamento com o id informado do banco de dados.
    //
    // No futuro, é melhor utilizar deleção lógica ao invés de absoluta.
    Entry::delete(&mut tr, id)?;
    tr.commit()?;

    Ok(())
}

#[get("/?<dia>&<periodo>")]
fn get_report(dia: i32, periodo: i32) -> Result<String> {
    // Abre a conexão com o banco de dados.
    let mut conn = get_conn()?;

    // Gera os relatórios de fluxo de caixa, com base no dia e período informados.
    let report = Report::generate(&mut conn, dia, periodo)?;

    Ok(report.print(dia, periodo))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![help])
        .mount(
            "/lancamento",
            routes![
                create_entry,
                get_all_entries,
                get_entries_by_value,
                get_entries_by_day,
                get_entries_by_origin,
                get_entries_by_destination,
                delete_entry
            ],
        )
        .mount("/relatorio", routes![get_report])
        .launch();
}
