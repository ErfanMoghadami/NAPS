mod app;
mod cli;
mod csv_reader;
mod input_files;
mod models;

fn main() {
    let args = cli::cli();
    app::app(&args);
    println!("{args:#?}")
}
