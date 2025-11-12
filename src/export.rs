use crate::models::report::Report;
use std::fs;
use std::path::Path;

pub fn export_report_to_csv(report: &Report, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut csv_content = String::new();

    // Header
    csv_content.push_str("Period,");
    csv_content.push_str(&report.period);
    csv_content.push('\n');

    // Data
    csv_content.push_str("Label,Value\n");
    for item in &report.data {
        csv_content.push_str(&format!("\"{}\",{}\n", item.label.replace("\"", "\"\""), item.value));
    }

    fs::write(file_path, csv_content)?;
    Ok(())
}

pub fn export_report_to_txt(report: &Report, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut txt_content = format!("Report Period: {}\n\n", report.period);
    txt_content.push_str("Data:\n");

    for item in &report.data {
        txt_content.push_str(&format!("{}: {}\n", item.label, item.value));
    }

    fs::write(file_path, txt_content)?;
    Ok(())
}
