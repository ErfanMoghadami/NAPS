use crate::cli::CliArgs;
use crate::csv_reader::{machine_csv_reader, operation_csv_reader, order_csv_reader};
use crate::input_files::input_validation_three_files;

pub fn app(args: &CliArgs) {
    let all_files_validation = input_validation_three_files(&args);

    if all_files_validation.is_empty() {
        println!("input validation is ok")
    } else {
        eprintln!("input validation is not ok");
        for error in all_files_validation {
            println!("{error}")
        }
        return;
    }

    let machines_result = machine_csv_reader(&args.machines);

    match machines_result {
        Ok(machines) => {
            println!("Machines loaded successfully:");
            println!("{machines:#?}");
        }
        Err(error) => {
            eprintln!("Failed to parse machines.csv: {error}");
        }
    }
    let orders_result = order_csv_reader(&args.orders);

    match orders_result {
        Ok(orders) => {
            println!("Orders loaded successfully:");
            println!("{orders:#?}");
        }
        Err(error) => {
            eprintln!("Failed to parse orders.csv: {error}");
        }
    }
    let operations_result = operation_csv_reader(&args.operations);

    match operations_result {
        Ok(operations) => {
            println!("Operations loaded successfully:");
            println!("{operations:#?}");
        }
        Err(error) => {
            eprintln!("Failed to parse orders.csv: {error}");
        }
    }
}
