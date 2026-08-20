/// Dados agregados da carteira do usuário, usados no dashboard.
///
/// Todos os valores monetários estão em R$ (reais).
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct PortfolioSummary {
    /// Valor de mercado atual de todos os ativos (quantidade × unit_value atual).
    pub patrimony: f64,
    /// Custo total de aquisição (quantidade × bought_for em cada operação BUY/SELL).
    pub invested: f64,
    /// Percentual de rentabilidade: (patrimony - invested) / invested × 100.
    pub profitability: f64,
    /// Número de ativos distintos na carteira.
    pub total_assets: i64,
    /// Número total de operações (BUY + SELL).
    pub total_operations: i64,
}

impl PortfolioSummary {
    /// Constrói um `PortfolioSummary` a partir dos componentes calculados no banco.
    pub fn new(patrimony: f64, invested: f64, total_assets: i64, total_operations: i64) -> Self {
        // Evita divisão por zero quando não há valor investido.
        let profitability = if invested == 0.0 {
            0.0
        } else {
            (patrimony - invested) / invested * 100.0
        };

        Self {
            patrimony,
            invested,
            profitability,
            total_assets,
            total_operations,
        }
    }

    /// Formata `patrimony` como string BRL (ex.: "R$ 25.430,00").
    pub fn format_patrimony(&self) -> String {
        format_brl(self.patrimony)
    }

    /// Formata `invested` como string BRL.
    pub fn format_invested(&self) -> String {
        format_brl(self.invested)
    }

    /// Formata `profitability` como "+15,59 %" ou "-3,21 %".
    pub fn format_profitability(&self) -> String {
        let sign = if self.profitability >= 0.0 { "+" } else { "" };
        format!("{}{:.2} %", sign, self.profitability).replace('.', ",")
    }
}

/// Converte um valor f64 para o formato monetário brasileiro.
pub fn format_brl(value: f64) -> String {
    // Separa a parte inteira e a decimal.
    let cents = (value * 100.0).round() as i64;
    let abs_cents = cents.unsigned_abs();
    let int_part = abs_cents / 100;
    let dec_part = abs_cents % 100;

    // Insere separadores de milhar na parte inteira.
    let int_str = int_part.to_string();
    let mut with_dots = String::new();
    for (i, c) in int_str.chars().rev().enumerate() {
        if i != 0 && i % 3 == 0 {
            with_dots.push('.');
        }
        with_dots.push(c);
    }
    let int_formatted: String = with_dots.chars().rev().collect();

    let sign = if cents < 0 { "-" } else { "" };
    format!("{}R$ {},{:02}", sign, int_formatted, dec_part)
}

#[derive(Debug, Serialize)]
pub struct PortfolioDistribution {
    pub name: String,
    pub value: f64,
    pub percentage: f64,
}

#[derive(Debug, Serialize)]
pub struct PortfolioHistoryPoint {
    pub date: String,
    pub value: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_brl() {
        assert_eq!(format_brl(25430.0), "R$ 25.430,00");
        assert_eq!(format_brl(22000.0), "R$ 22.000,00");
        assert_eq!(format_brl(1000000.5), "R$ 1.000.000,50");
    }

    #[test]
    fn test_profitability_positive() {
        let s = PortfolioSummary::new(25430.0, 22000.0, 3, 8);
        // (25430 - 22000) / 22000 * 100 ≈ 15.59%
        assert!((s.profitability - 15.59).abs() < 0.01);
        assert!(s.format_profitability().starts_with('+'));
    }

    #[test]
    fn test_profitability_zero_invested() {
        let s = PortfolioSummary::new(0.0, 0.0, 0, 0);
        assert_eq!(s.profitability, 0.0);
    }
}
