mod app;
mod cli;
mod csv_reader;
mod input_files;
mod models;
mod data_validation;
mod business_rules;

fn main() {
    let args = cli::cli();
    app::app(&args);
    println!("{args:#?}")
}
