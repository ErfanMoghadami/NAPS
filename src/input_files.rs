use std::path::Path;

use crate::cli::CliArgs;

pub fn input_validation_one_file(name: &str, pathname: &Path) -> Option<String> {
    if !pathname.exists() {
        return Some(format!("{name} does not exist"));
    }
    if !pathname.is_file() {
        return Some(format!("{name} is not a file"));
    }
    None
}

pub fn input_validation_three_files(args: &CliArgs) -> Vec<String> {
    let mut errors: Vec<String> = Vec::new();

    if let Some(error) = input_validation_one_file("machines.csv", &args.machines) {
        errors.push(error);
    }
    if let Some(error) = input_validation_one_file("orders.csv", &args.orders) {
        errors.push(error);
    }
    if let Some(error) = input_validation_one_file("operations.csv", &args.operations) {
        errors.push(error);
    }
    errors
}
