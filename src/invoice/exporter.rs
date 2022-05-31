use crate::invoice::Invoice;
use handlebars::{
    to_json, Context, Handlebars, Helper, HelperResult, Output, RenderContext, RenderError,
};
use serde_json::value::Map;
use wkhtmltopdf::*;

const INVOICE_TEMPLATE_NAME: &str = "invoice";

fn inc_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h
        .param(0)
        .ok_or(RenderError::new("Unable to retrieve value"))?;
    if let Some(val) = param.value().as_i64() {
        out.write(format!("{}", val + 1).as_str())?;
        Ok(())
    } else {
        Err(RenderError::new("Unable to cast to str"))
    }
}

fn to_fixed_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let value = h
        .param(0)
        .ok_or(RenderError::new("Unable to retrieve value"))?;
    let padding = h
        .param(1)
        .ok_or(RenderError::new("Unable to retrieve padding"))?;
    if let Some(padding) = padding.value().as_u64() {
        if let Some(value) = value.value().as_f64() {
            out.write(format!("{:.precision$}", value, precision = padding as usize).as_str())?;
            Ok(())
        } else {
            Err(RenderError::new("Unable to cast value to f_64"))
        }
    } else {
        Err(RenderError::new("Unable to cast padding to u_64"))
    }
}

fn breaklines_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h
        .param(0)
        .ok_or(RenderError::new("Unable to retrieve value"))?;
    if let Some(val) = param.value().as_str() {
        out.write(val.replace("\n", "<br/>").as_str())?;
        Ok(())
    } else {
        Err(RenderError::new("Unable to cast to str"))
    }
}

fn generate_invoice(invoice_data: &Invoice, template_path: &str) -> String {
    let mut handlebars = Handlebars::new();
    handlebars.register_helper("breaklines", Box::new(breaklines_helper));
    handlebars.register_helper("inc", Box::new(inc_helper));
    handlebars.register_helper("to_fixed", Box::new(to_fixed_helper));
    handlebars
        .register_template_file(INVOICE_TEMPLATE_NAME, template_path)
        .unwrap();

    let mut data = Map::new();
    data.insert("invoice".to_string(), to_json(invoice_data));
    data.insert(
        "amount_total".to_string(),
        to_json(invoice_data.get_total_items_amount()),
    );
    return handlebars.render(INVOICE_TEMPLATE_NAME, &data).unwrap();
}

fn save_to_pdf(html_data: String, result_path: &str) {
    let pdf_app = PdfApplication::new().expect("Failed to init PDF application");
    let mut pdfout = pdf_app
        .builder()
        .build_from_html(&html_data)
        .expect("failed to build pdf");

    pdfout.save(result_path).expect("failed to save pdf");
}

impl Invoice {
    pub fn export_as_pdf(&self, template_path: &str, output_file: &str) {
        let result = generate_invoice(self, template_path);
        save_to_pdf(result, output_file);
    }
}
