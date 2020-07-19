#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    // API usage
    "Hello, world!"
}

#[put("/")]
fn create_entry() -> &'static str {
    "Criou entrada"
}

#[get("/")]
fn get_entry() -> &'static str {
    "Buscou entrada"
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
        .mount("/", routes![index])
        .mount("/lancamento", routes![create_entry, get_entry, delete_entry])
        .mount("/relatorio", routes![get_report])
        .launch();
}
