#[macro_use]
extern crate failure;

pub mod err;

pub use err::Result;

use postgres::Client;
use postgres_types::FromSql;

#[derive(FromSql, Debug)]
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
#[derive(Debug)]
pub struct Entry {
    pub id: i32, // TODO: mudar para UUID
    pub valor: i32,
    pub dia: i32,
    pub class: Classificacao,
    pub origem: Option<i32>,
    pub destino: Option<i32>,
}

impl Entry {
    pub fn new(
        id: i32,
        valor: i32,
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
            let id: i32 = row.get(0);
            let valor: i32 = row.get(1);
            let dia: i32 = row.get(2);
            let class: Classificacao = row.get(3);
            let origem: Option<i32> = row.get(4);
            let destino: Option<i32> = row.get(5);

            let entry = Entry::new(id, valor, dia, class, origem, destino);

            entries.push(entry);
        }

        return Ok(entries);
    }

    pub fn delete() -> Result<()> {
        unimplemented!()
    }
}
