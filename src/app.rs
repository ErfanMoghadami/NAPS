use std::fmt::Alignment::Left;

use crate::cli::CliArgs;
use crate::csv_reader::{machine_csv_reader, operation_csv_reader, order_csv_reader};
use crate::data_validation::{
    self, data_validation_machine, data_validation_operations, data_validation_orders,
    validation_refrence,
};
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
    let machines_data = match machines_result {
        Ok(machines) => {
            let errors = data_validation_machine(&machines);

            if errors.is_empty() {
                println!("machine data is valid.");
                Some(machines)
            } else {
                for error in errors {
                    eprintln!("{error}")
                }
                None
            }
        }

        Err(error) => {
            eprintln!("Failed to parse machines.csv: {error}");
            None
        }
    };

    let orders_result = order_csv_reader(&args.orders);
    let order_data = match orders_result {
        Ok(orders) => {
            let errors = data_validation_orders(&orders);
            if errors.is_empty() {
                println!("order data is valid.");
                Some(orders)
            } else {
                for error in errors {
                    eprintln!("{error}")
                }
                None
            }
        }
        Err(error) => {
            eprintln!("Failed to parse orders.csv: {error}");
            None
        }
    };

    let operations_result = operation_csv_reader(&args.operations);
    let operations_data = match operations_result {
        Ok(operations) => {
            let errors = data_validation_operations(&operations);
            if errors.is_empty() {
                println!("operation data is valid.");
                Some(operations)
            } else {
                for error in errors {
                    eprintln!("{error}")
                }
                None
            }
        }
        Err(error) => {
            eprintln!("Failed to parse orders.csv: {error}");
            None
        }
    };

    // match validation_ref_result {
    //     Ok(refrences) => {
    //         let error = validation_refrence(machines_data, order_data, operations_data);
    //     }
    // }

    match (machines_data, order_data, operations_data) {
        (Some(machines), Some(orders), Some(operations)) => {
            let errors = validation_refrence(&machines, &orders, &operations);
        }
        _ => {
            eprintln!("cannont validate references:one or more datasets are missing")
        }
    }
}
