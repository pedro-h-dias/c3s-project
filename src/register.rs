use crate::{err::Result, Classificacao};
use postgres::Transaction;
use serde_derive::Deserialize;

/// Estrutura auxiliar para um novo lançamento.
///
/// Guarda as informações de novos lançamentos enquanto
/// eles ainda não foram persistidos no banco de dados.
#[derive(Debug, Deserialize)]
pub struct NewEntry {
    valor: f32,
    dia: i32,
    class: Classificacao,
    origem: Option<i32>,
    destino: Option<i32>,
}

impl NewEntry {
    /// Persiste o novo lançamento no banco de dados.
    pub fn persist(&self, conn: &mut Transaction) -> Result<()> {
        if self.origem.is_some() {
            conn.execute(
                "
                INSERT INTO erp (valor, dia, class, origem)
                VALUES($1, $2, $3, $4)",
                &[&self.valor, &self.dia, &self.class, &self.origem],
            )?;
        } else if self.destino.is_some() {
            conn.execute(
                "
                INSERT INTO erp (valor, dia, class, destino)
                VALUES($1, $2, $3, $4)",
                &[&self.valor, &self.dia, &self.class, &self.origem],
            )?;
        };

        Ok(())
    }

    /// Verifica se a entrada é válida.
    ///
    /// Uma entrada é válida se possui origem ou destino (mas não os dois) entre
    /// 1 e 10, e se o dia informado está entre 1 e 30.
    pub fn is_valid(&self) -> bool {
        (self
            .origem
            .map(|i| i >= 1 && i <= 10)
            .or(Some(false))
            .unwrap()
            | self
                .destino
                .map(|i| i >= 1 && i <= 10)
                .or(Some(false))
                .unwrap())
            && (self.dia >= 0 && self.dia <= 30)
    }
}

#[cfg(test)]
mod tests {
    use super::NewEntry;
    use postgres::{Client, NoTls};

    #[test]
    fn deserialize_new_entry() {
        let data = r#"
            {
                "valor": 13.37,
                "dia": 25,
                "class": "Receita",
                "origem": 2
            }"#;

        let new_entry: Result<NewEntry, _> = serde_json::from_str(data);

        assert!(new_entry.is_ok());
    }

    #[test]
    fn create_entry() {
        let data = r#"
            {
                "valor": 13.37,
                "dia": 25,
                "class": "Receita",
                "origem": 2
            }"#;

        let new_entry: NewEntry = serde_json::from_str(data).expect("Failed to deserialize");

        let mut conn = Client::connect("host=localhost dbname=erp-database user=locutor", NoTls)
            .expect("Failed to connect to database.");

        let mut tr = conn.transaction().expect("Failed to initiate transaction");

        let result = new_entry.persist(&mut tr);
        tr.rollback().unwrap();

        assert!(result.is_ok());
    }
}
