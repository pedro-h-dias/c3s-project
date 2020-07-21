#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;

pub mod database;
pub mod err;
pub mod register;

use err::{ErpError, Result};
pub use register::NewEntry;

use postgres::{GenericClient, Transaction};
use postgres_types::{FromSql, ToSql};
use uuid::Uuid;

/// Enum que representa a Classificação do lançamento.
///
/// Um lançamento pode ser um custo, uma despesa ou uma receita.
#[derive(Debug, ToSql, FromSql, Serialize, Deserialize)]
#[postgres(name = "classificacao")]
pub enum Classificacao {
    #[postgres(name = "receita")]
    Receita,
    #[postgres(name = "custo")]
    Custo,
    #[postgres(name = "despesa")]
    Despesa,
}

/// Estrutura que representa um lançamento.
///
/// Além dos campos obrigatórios, precisa conter a origem
/// ou o destino da transação.
#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    id: Uuid,
    valor: f32,
    dia: i32,
    class: Classificacao,
    origem: Option<i32>,
    destino: Option<i32>,
}

impl Entry {
    /// Constrói um lançamento a partir de seus campos.
    pub fn new(
        id: Uuid,
        valor: f32,
        dia: i32,
        class: Classificacao,
        origem: Option<i32>,
        destino: Option<i32>,
    ) -> Self {
        Self {
            id,
            valor,
            dia,
            class,
            origem,
            destino,
        }
    }

    /// Busca lançamentos por valor, dia, origem ou destino.
    ///
    /// É esperado um int32 para dia, origem ou destino.
    /// Para valor, é esperado um f32.
    pub fn get_by(
        conn: &mut impl GenericClient,
        param: &str,
        value_int: Option<i32>,
        value_float: Option<f32>,
    ) -> Result<Vec<Self>> {
        // Conectar no banco e fazer a query pelo parâmetro informado.
        let rows = if value_int.is_some() {
            conn.query(
                format!(
                    "SELECT id, valor, dia, class, origem, destino FROM erp WHERE {} = $1",
                    param
                )
                .as_str(),
                &[&value_int.unwrap()],
            )?
        } else if value_float.is_some() {
            conn.query(
                format!(
                    "SELECT id, valor, dia, class, origem, destino FROM erp WHERE {} = $1",
                    param
                )
                .as_str(),
                &[&value_float.unwrap()],
            )?
        } else {
            // O código não deve chegar aqui em nenhum caso.
            unreachable!();
        };

        // Retorna NotFound caso nenhum resultado seja encontrado.
        if rows.is_empty() {
            return Err(ErpError::NotFound);
        }

        // Para cada linha, gerar structs, adicionar em um vetor, e retornar.
        let mut entries: Vec<_> = Vec::with_capacity(rows.len());
        for row in rows {
            let id: Uuid = row.get(0);
            let valor: f32 = row.get(1);
            let dia: i32 = row.get(2);
            let class: Classificacao = row.get(3);
            let origem: Option<i32> = row.get(4);
            let destino: Option<i32> = row.get(5);

            entries.push(Entry::new(id, valor, dia, class, origem, destino));
        }

        return Ok(entries);
    }

    /// Busca todos os lançamentos no banco de dados.
    pub fn get_all(conn: &mut impl GenericClient) -> Result<Vec<Self>> {
        // Conectar no banco e fazer a query.
        let rows = conn.query(
            "SELECT id, valor, dia, class, origem, destino FROM erp",
            &[],
        )?;

        // Retorna NotFound caso nenhum resultado seja encontrado.
        if rows.is_empty() {
            return Err(ErpError::NotFound);
        }

        // Para cada linha, gerar structs, adicionar em um vetor, e retornar.
        let mut entries: Vec<_> = Vec::with_capacity(rows.len());
        for row in rows {
            let id: Uuid = row.get(0);
            let valor: f32 = row.get(1);
            let dia: i32 = row.get(2);
            let class: Classificacao = row.get(3);
            let origem: Option<i32> = row.get(4);
            let destino: Option<i32> = row.get(5);

            entries.push(Entry::new(id, valor, dia, class, origem, destino));
        }

        return Ok(entries);
    }

    /// Deleta um lançamento dado seu identificador.
    ///
    /// Retorna NotFound caso não haja deleções.
    pub fn delete(conn: &mut Transaction, id: Uuid) -> Result<()> {
        let deleted = conn.execute("DELETE FROM erp WHERE id = $1", &[&id])?;

        if deleted == 0 {
            return Err(ErpError::NotFound);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use postgres::{Client, NoTls};
    use serde_json::json;
    use std::str::FromStr;

    #[test]
    fn delete_entry() {
        let mut conn = Client::connect("host=localhost dbname=erp-database user=locutor", NoTls)
            .expect("Failed to connect to database.");
        let mut tr = conn.transaction().expect("Failed to initiate transaction");

        let data = json!({
            "id": "acbbef75-09a9-4acc-8d12-47d99862b37e",
            "valor": 13.37,
            "dia": 25,
            "class": "Receita",
            "origem": 2
        });

        let entry: Entry = serde_json::from_value(data).unwrap();

        tr.execute(
            "INSERT INTO erp (id,valor,dia,class,origem) VALUES ($1, $2, $3, $4, $5)",
            &[
                &entry.id,
                &entry.valor,
                &entry.dia,
                &entry.class,
                &entry.origem,
            ],
        )
        .expect("Failed to insert test data into the database");

        let id =
            Uuid::from_str("acbbef75-09a9-4acc-8d12-47d99862b37e").expect("Failed to parse UUID");

        let result = Entry::delete(&mut tr, id);

        tr.rollback().unwrap();

        assert!(result.is_ok());
    }

    #[test]
    fn get_entries() {
        let mut conn = Client::connect("host=localhost dbname=erp-database user=locutor", NoTls)
            .expect("Failed to connect to database.");

        let entries = Entry::get_by(&mut conn, "dia", Some(25), None);

        assert!(entries.is_ok());

        for entry in entries.unwrap() {
            println!("{:?}", entry);
        }
    }
}
