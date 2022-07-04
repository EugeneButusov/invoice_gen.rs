use crate::clockify::ClockifySettings;
use crate::util::date_serialize;
use chrono::{Date, Utc};

pub mod exporter;
pub mod factory;

#[derive(Serialize)]
pub struct InvoiceBlueprint {
    pub contract_number: Option<u16>,
    #[serde(with = "date_serialize")]
    pub contract_date: Date<Utc>,
    pub recipient_data: String,
    pub payer_data: String,
    pub payment_instructions: String,
    pub currency: String,
    pub signature: String,
    pub salary: f64,
    pub clockify_settings: ClockifySettings,
}

#[derive(Serialize)]
pub struct InvoiceItem {
    pub description: String,
    pub amount: f64,
}

#[derive(Serialize)]
pub struct Invoice {
    pub signature: String,
    pub contract_number: Option<u16>,
    #[serde(with = "date_serialize")]
    pub contract_date: Date<Utc>,
    #[serde(with = "date_serialize")]
    pub invoiced_at: Date<Utc>,
    pub invoice_number: u16,
    pub recipient_data: String,
    pub payer_data: String,
    pub payment_instructions: String,
    pub currency: String,
    pub items: Vec<InvoiceItem>,
}

impl Invoice {
    pub fn get_total_items_amount(&self) -> f64 {
        let mut result = 0 as f64;
        for item in &self.items {
            result += item.amount;
        }
        result
    }
}
