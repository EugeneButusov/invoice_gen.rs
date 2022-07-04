use crate::clockify::ClockifySettings;
use crate::invoice::{Invoice, InvoiceBlueprint, InvoiceItem};
use chrono::prelude::*;
use chrono::Duration;
use chrono::Weekday::{Sat, Sun};
use std::ops::Add;
use toml;

fn total_months(date: Date<Utc>) -> u32 {
    (date.year() as u32) * 12 + date.month()
}

// TODO: not efficient enough, better to replace with formula
fn calc_work_days(month: u32, year: i32) -> i8 {
    let mut date = Utc.ymd(year, month, 1);
    let mut work_days = 0;

    while date.month() == month {
        if date.weekday() != Sat && date.weekday() != Sun {
            work_days += 1;
        }
        date = date.add(Duration::days(1));
    }

    work_days
}

impl InvoiceBlueprint {
    pub fn from_file(path: &str) -> InvoiceBlueprint {
        let invoice_blueprint_data = toml::from_str::<toml::Value>(
            std::fs::read_to_string(path)
                .expect("Blueprint::new -> Cannot read blueprint file")
                .as_str(),
        )
        .expect("Blueprint::new -> Bad blueprint file format");

        let invoice_data_contract_date = invoice_blueprint_data["contract"]
            .as_table()
            .expect("Blueprint::new -> Unable to read contract date")["date"]
            .as_datetime()
            .expect("Blueprint::new -> Unable to read contract date");

        let contract_date = Utc.ymd(
            invoice_data_contract_date.date.as_ref().unwrap().year as i32,
            invoice_data_contract_date.date.as_ref().unwrap().month as u32,
            invoice_data_contract_date.date.as_ref().unwrap().day as u32,
        );

        let clockify_data = invoice_blueprint_data["clockify"]
            .as_table()
            .expect("Blueprint::new -> Unable to read clockify settings");

        InvoiceBlueprint {
            contract_number: None,
            contract_date,
            recipient_data: invoice_blueprint_data["recipient"]
                .as_table()
                .expect("Blueprint::new -> Unable to read recipient data")["data"]
                .as_str()
                .expect("Blueprint::new -> Unable to read recipient data")
                .to_string(),
            payer_data: invoice_blueprint_data["payer"]
                .as_table()
                .expect("Blueprint::new -> Unable to read payer data")["data"]
                .as_str()
                .expect("Blueprint::new -> Unable to read payer data")
                .to_string(),
            payment_instructions: invoice_blueprint_data["recipient"]
                .as_table()
                .expect("Blueprint::new -> Unable to read payment instructions")
                ["payment_instructions"]
                .as_str()
                .expect("Blueprint::new -> Unable to read payment instructions")
                .to_string(),
            currency: invoice_blueprint_data["contract"]
                .as_table()
                .expect("Blueprint::new -> Unable to read contract currency")["currency"]
                .as_str()
                .expect("Blueprint::new -> Unable to read contract currency")
                .to_string(),
            signature: invoice_blueprint_data["invoice"]
                .as_table()
                .expect("Blueprint::new -> Unable to read invoice signature")["signature"]
                .as_str()
                .expect("Blueprint::new -> Unable to read invoice signature")
                .to_string(),
            salary: invoice_blueprint_data["contract"]
                .as_table()
                .expect("Blueprint::new -> Unable to read contract salary")["salary"]
                .as_float()
                .expect("Blueprint::new -> Unable to read contract salary"),
            clockify_settings: ClockifySettings {
                api_key: clockify_data["api_key"]
                    .as_str()
                    .expect("Blueprint::new -> Unable to read clockify api_key")
                    .to_string(),
                workspace_id: clockify_data["workspace_id"]
                    .as_str()
                    .expect("Blueprint::new -> Unable to read clockify workspace_id")
                    .to_string(),
                user_id: clockify_data["user_id"]
                    .as_str()
                    .expect("Blueprint::new -> Unable to read clockify user_id")
                    .to_string(),
            },
        }
    }
}

impl Invoice {
    pub fn from_blueprint(
        blueprint: &InvoiceBlueprint,
        invoiced_at: Date<Utc>,
        items: Vec<InvoiceItem>,
    ) -> Invoice {
        Invoice {
            invoiced_at,
            invoice_number: (total_months(invoiced_at) - total_months(blueprint.contract_date) + 1)
                as u16,
            contract_number: None,
            contract_date: blueprint.contract_date,
            recipient_data: blueprint.recipient_data.clone(),
            payer_data: blueprint.payer_data.clone(),
            payment_instructions: blueprint.payment_instructions.clone(),
            currency: blueprint.currency.clone(),
            signature: blueprint.signature.clone(),
            items,
        }
    }
}

impl InvoiceItem {
    pub fn new_for_daily_work(invoiced_at: Date<Utc>, salary: f64, days_off: u8) -> InvoiceItem {
        let total_days = calc_work_days(invoiced_at.month(), invoiced_at.year());
        let worked_days = total_days - days_off as i8;

        InvoiceItem {
            amount: salary * worked_days as f64 / total_days as f64,
            description: format!(
                "{} ({} / {} work days)",
                invoiced_at.format("%B, %Y"),
                worked_days,
                total_days
            ),
        }
    }

    pub fn new_for_clockify_period(_from: &DateTime<Utc>, _to: &DateTime<Utc>) -> InvoiceItem {
        InvoiceItem {
            amount: 0 as f64,
            description: "TBD".to_string(),
        }
    }
}
