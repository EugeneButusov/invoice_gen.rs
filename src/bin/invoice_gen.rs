use chrono::prelude::*;
use clap::Parser;
use invoice_gen::clockify::client::Client;
use invoice_gen::invoice::{Invoice, InvoiceBlueprint, InvoiceItem};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Blueprint file path
    #[clap(short, long)]
    blueprint_path: String,

    /// Date invoice counts time from
    #[clap(short, long)]
    from: DateTime<FixedOffset>,

    /// Date invoice counts time to
    #[clap(short, long)]
    to: DateTime<FixedOffset>,

    /// Date invoice generated at
    #[clap(short, long)]
    invoiced_at: DateTime<Utc>,

    /// Output invoice file
    #[clap(short, long)]
    output: String,
}

fn main() {
    let args = Args::parse();
    let invoiced_at = args.invoiced_at.date();

    let invoice_blueprint = InvoiceBlueprint::from_file(args.blueprint_path.as_str());

    let clockify_client = Client::new(invoice_blueprint.clockify_settings.clone());
    let duration = clockify_client.get_duration_for_period(&invoice_blueprint.clockify_settings.user_id, &args.from, &args.to);
    print!("Invoiced duration: {}", &duration);

    let invoice = Invoice::from_blueprint(
        &invoice_blueprint,
        invoiced_at,
        vec![InvoiceItem::new_for_daily_work(
            invoiced_at,
            invoice_blueprint.salary,
            0,
        )],
    );

    invoice.export_as_pdf(args.output.as_str());
}
