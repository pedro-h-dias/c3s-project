use crate::err::Result;
use crate::{Classificacao, Entry};
use postgres::Transaction;
use serde_derive::Deserialize;

// Estrutura auxiliar para um novo lancamento.
//
// Guarda as informacoes de novos lancamentos enquanto
// eles ainda nao foram persistidos no banco de dados.
#[derive(Debug, Deserialize)]
pub struct NewEntry {
    valor: f32,
    dia: i32,
    class: Classificacao,
    origem: Option<i32>,
    destino: Option<i32>,
}

impl NewEntry {
    // Persiste o novo lancamento no banco de dados.
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

    // Verifica se possui origem ou destino.
    pub fn is_valid(&self) -> bool {
        self.origem.is_some() || self.destino.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::NewEntry;
    use postgres::{Client, NoTls};
    use serde_json::Value;

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

        // Abre a conexao com o banco de dados
        let mut conn = Client::connect("host=localhost dbname=erp-database user=locutor", NoTls)
            .expect("Failed to connect to database.");

        let mut tr = conn.transaction().expect("Failed to initiate transaction");

        let result = new_entry.persist(&mut tr);
        tr.rollback().unwrap();

        result.unwrap();
        //assert!(result.is_ok());
    }
}
