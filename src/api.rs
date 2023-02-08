use std::{collections::HashMap, fmt};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CurrenciesResponse {
    rates: HashMap<String, f64>
}

#[derive(Debug)]
pub enum CurrencyError {
    CurrencyNotFound,
    ReqwestError(reqwest::Error),
}

impl From<reqwest::Error> for CurrencyError {
    fn from(err: reqwest::Error) -> Self {
        CurrencyError::ReqwestError(err)
    }
}

impl fmt::Display for CurrencyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CurrencyError::CurrencyNotFound => write!(f, "Currency not found"),
            CurrencyError::ReqwestError(err) => write!(f, "Request error: {}", err),
        }
    }
}

pub async fn get_currency_rate(from: &str, to: &str) -> Result<(f64, f64), CurrencyError> {
    let client = reqwest::Client::new();
    let response = client.get("https://openexchangerates.org/api/latest.json")
        .query(&[("app_id", std::env::var("OPEN_EXCHANGE_APP_ID").unwrap())])
        .send()
        .await
        .map_err(CurrencyError::ReqwestError)?;


    let currencies =response.json::<CurrenciesResponse>().await?;
    

    let rate_from = match currencies.rates.get(from) {
        Some(r) => r,
        None => return Err(CurrencyError::CurrencyNotFound)
    };

    let rate_to = match currencies.rates.get(to) {
        Some(r) => r,
        None => return Err(CurrencyError::CurrencyNotFound)
    };

    Ok((*rate_from, *rate_to))
}