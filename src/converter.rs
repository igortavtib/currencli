use crate::api::{get_currency_rate, CurrencyError};

pub async fn convert(from: &str, to: &str, value: &f64) -> Result<f64, CurrencyError> {
    let rates = get_currency_rate(from, to).await?;

    Ok((rates.1 * value) / rates.0)
}