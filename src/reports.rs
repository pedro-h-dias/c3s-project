use crate::{err::Result, Classificacao, Entry};

use postgres::GenericClient;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Report {
    saldo_atual: f32,
    receitas: f32,
    custos: f32,
    despesas: f32,
}

impl Report {
    /// Retorna report dos últimos `periodo` dias a partir de `dia`.
    pub fn generate(conn: &mut impl GenericClient, dia: i32, periodo: i32) -> Result<Self> {
        let entries = Entry::get_all(conn)?;

        let receitas = entries
            .iter()
            .filter(|e| e.dia <= dia && e.dia > (dia - periodo))
            .filter(|e| e.class == Classificacao::Receita)
            .map(|e| e.valor)
            .sum();

        let custos = entries
            .iter()
            .filter(|e| e.dia <= dia && e.dia > (dia - periodo))
            .filter(|e| e.class == Classificacao::Custo)
            .map(|e| e.valor)
            .sum();

        let despesas = entries
            .iter()
            .filter(|e| e.dia <= dia && e.dia > (dia - periodo))
            .filter(|e| e.class == Classificacao::Despesa)
            .map(|e| e.valor)
            .sum();

        let total_saidas: f32 = entries
            .iter()
            .filter(|e| e.class == Classificacao::Despesa || e.class == Classificacao::Custo)
            .map(|e| e.valor)
            .sum();

        let total_entradas: f32 = entries
            .iter()
            .filter(|e| e.class == Classificacao::Receita)
            .map(|e| e.valor)
            .sum();

        let saldo_atual = total_entradas - total_saidas;

        Ok(Self {
            saldo_atual,
            despesas,
            custos,
            receitas,
        })
    }

    /// Imprime o relatório, deixando em um formato de mais fácil leitura.
    pub fn print(self, dia: i32, periodo: i32) -> String {
        let total_entradas = self.receitas;
        let total_saidas = self.despesas + self.custos;
        let saldo_inicial = self.saldo_atual - total_entradas + total_saidas;

        format!(
            r#"
            FLUXO DE CAIXA DO DIA {} NO PERIODO DE {} DIA(S)

            SALDO INICIAL: {:.2}
            
            TOTAL DESPESAS: {:.2}
            TOTAL CUSTOS: {:.2}
            TOTAL DAS SAÍDAS: {:.2}
            
            TOTAL RECEITAS: {:.2}
            TOTAL DAS ENTRADAS: {:.2}

            SALDO ATUAL: {:.2}
            "#,
            dia,
            periodo,
            saldo_inicial,
            self.despesas,
            self.custos,
            total_saidas,
            self.receitas,
            total_entradas,
            self.saldo_atual
        )
        .to_owned()
    }
}
