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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn existing_file_should_be_valid() {
        let result = input_validation_one_file("Cargo.toml", Path::new("Cargo.toml"));

        assert_eq!(result, None);
    }

    #[test]
    fn missing_file_should_return_error() {
        let result = input_validation_one_file("missing.csv", Path::new("missing.csv"));

        assert!(result.is_some());
    }

    #[test]
    fn directory_should_return_error() {
        let result = input_validation_one_file("src", Path::new("src"));

        assert!(result.is_some());
    }
}
