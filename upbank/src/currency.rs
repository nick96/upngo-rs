use crate::iso4217;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Money {
    pub currency_code: iso4217::CurrencyCode,
    pub value: String,
    pub value_in_base_units: i64,
}