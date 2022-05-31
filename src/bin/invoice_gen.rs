use chrono::prelude::*;
use clap::Parser;
use invoice_gen::invoice::{Invoice, InvoiceBlueprint, InvoiceItem};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Blueprint file path
    #[clap(short, long)]
    blueprint_path: String,

    /// Date invoice generated at
    #[clap(short, long)]
    invoiced_at: DateTime<Utc>,

    /// Output invoice file
    #[clap(short, long)]
    output: String,

    /// Amount of non-working days
    #[clap(short, long, default_value_t = 0)]
    days_off: u8,
}

fn main() {
    let args = Args::parse();
    let invoiced_at = args.invoiced_at.date();

    let invoice_blueprint = InvoiceBlueprint::from_file(args.blueprint_path.as_str());

    let invoice = Invoice::from_blueprint(
        &invoice_blueprint,
        invoiced_at,
        vec![InvoiceItem::new_for_daily_work(
            invoiced_at,
            invoice_blueprint.salary,
            args.days_off,
        )],
    );

    invoice.export_as_pdf(args.output.as_str());
}
