#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;

pub mod err;
pub mod register;

pub use err::Result;
pub use register::NewEntry;

use postgres::{Client, Transaction};
use postgres_types::{FromSql, ToSql};
use uuid::Uuid;

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

// Estrutura que representa um lancamento.
//
// Alem dos campos obrigatorios, precisa conter a origem
// ou o destino da transacao.
#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    pub id: Uuid,
    pub valor: f32,
    pub dia: i32,
    pub class: Classificacao,
    pub origem: Option<i32>,
    pub destino: Option<i32>,
}

impl Entry {
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

    pub fn get_by(conn: &mut Client, param: &str, value: i32) -> Result<Vec<Self>> {
        // Conectar no banco e fazer a quary pelo parametro informado
        let rows = conn
            .query(
                "SELECT entry_id, valor, dia, class, origem, destino FROM erp WHERE $1 = $2",
                &[&param, &value],
            )
            .expect("Failed to query database");

        // Para cada linha, gerar structs, adicionar em um vetor, e retornar
        let mut entries: Vec<_> = Vec::new();
        for row in rows {
            let id: Uuid = row.get(0);
            let valor: f32 = row.get(1);
            let dia: i32 = row.get(2);
            let class: Classificacao = row.get(3);
            let origem: Option<i32> = row.get(4);
            let destino: Option<i32> = row.get(5);

            let entry = Entry::new(id, valor, dia, class, origem, destino);

            entries.push(entry);
        }

        return Ok(entries);
    }

    pub fn delete(conn: &mut Transaction, id: Uuid) -> Result<()> {
        conn.execute("DELETE FROM erp WHERE id = $1", &[&id])?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use postgres::NoTls;
    use serde_json::json;
    use std::str::FromStr;

    #[test]
    fn delete_entry() {
        // Abre a conexao com o banco de dados
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
                &entry.origem
            ],
        )
        .unwrap();

        let id =
            Uuid::from_str("acbbef75-09a9-4acc-8d12-47d99862b37e").expect("Failed to parse UUID");

        let result = Entry::delete(&mut tr, id);

        tr.rollback().unwrap();

        assert!(result.is_ok());
    }
}
