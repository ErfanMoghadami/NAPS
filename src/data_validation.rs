use crate::models::{Company, Machine, Operation, Order};
use std::collections::HashSet;

pub fn data_validation_machine(machines: &[Machine]) -> Vec<String> {
    let mut errors = Vec::new();

    for (index, machine) in machines.iter().enumerate() {
        let row = index + 2;

        if machine.machine_id.trim().is_empty() {
            errors.push(format!("Row {row}: machine_id cannot be empty"));
        }

        if machine.company_id.trim().is_empty() {
            errors.push(format!("Row {row}: company_id cannot be empty"));
        }

        if machine.production_site_id.trim().is_empty() {
            errors.push(format!("Row {row}: production_site_id cannot be empty"));
        }

        if machine.code.trim().is_empty() {
            errors.push(format!("Row {row}: machine code cannot be empty"));
        }

        if machine.name.trim().is_empty() {
            errors.push(format!("Row {row}: machine name cannot be empty"));
        }
    }

    errors
}

pub fn data_validation_orders(orders: &[Order]) -> Vec<String> {
    let mut errors = Vec::new();

    for (index, order) in orders.iter().enumerate() {
        let row = index + 2;

        if order.order_id.trim().is_empty() {
            errors.push(format!("Row {row}: order_id cannot be empty"));
        }

        if order.company_id.trim().is_empty() {
            errors.push(format!("Row {row}: company_id cannot be empty"));
        }

        if order.product_id.trim().is_empty() {
            errors.push(format!("Row {row}: product_id cannot be empty"));
        }

        if order.routing_id.trim().is_empty() {
            errors.push(format!("Row {row}: routing_id cannot be empty"));
        }

        if order.quantity <= 0 {
            errors.push(format!("Row {row}: quantity must be greater than zero"));
        }
    }

    errors
}

pub fn data_validation_operations(operations: &[Operation]) -> Vec<String> {
    let mut errors = Vec::new();

    for (index, operation) in operations.iter().enumerate() {
        let row = index + 2;

        if operation.operation_id.trim().is_empty() {
            errors.push(format!("Row {row}: operation_id cannot be empty"));
        }

        if operation.order_id.trim().is_empty() {
            errors.push(format!("Row {row}: order_id cannot be empty"));
        }

        if operation.routing_step_id.trim().is_empty() {
            errors.push(format!("Row {row}: routing_step_id cannot be empty"));
        }

        if let Some(machine_id) = &operation.fixed_machine_id {
            if machine_id.trim().is_empty() {
                errors.push(format!("Row {row}: fixed_machine_id cannot be empty"));
            }
        }

        if operation.setup_time_seconds < 0 {
            errors.push(format!("Row {row}: setup_time_seconds cannot be negative"));
        }

        if operation.processing_time_per_unit_seconds <= 0 {
            errors.push(format!(
                "Row {row}: processing_time_per_unit_seconds must be greater than zero"
            ));
        }

        if operation.sequence == 0 {
            errors.push(format!("Row {row}: sequence must be greater than zero"));
        }
    }

    errors
}

pub fn data_validation_companies(companies: &[Company]) -> Vec<String> {
    let mut errors = Vec::new();
    let mut seen_ids = HashSet::new();

    for company in companies {
        if company.company_id.trim().is_empty() {
            errors.push("Company ID cannot be empty".to_string());
        }

        if !seen_ids.insert(company.company_id.clone()) {
            errors.push(format!("Duplicate company ID: {}", company.company_id));
        }

        if company.name.trim().is_empty() {
            errors.push(format!(
                "Company name cannot be empty (ID: {})",
                company.company_id
            ));
        }
    }

    errors
}

pub fn validation_refrence(
    machines: &[Machine],
    orders: &[Order],
    operations: &[Operation],
    companies: &[Company],
) -> Vec<String> {
    let mut errors = Vec::new();

    // Operation -> Order
    for operation in operations.iter() {
        if !orders
            .iter()
            .any(|order| order.order_id == operation.order_id)
        {
            errors.push(format!(
                "Order {} referenced by operation {} does not exist",
                operation.order_id, operation.operation_id
            ));
        }

        // Operation -> Fixed Machine (only when assigned)
        if let Some(machine_id) = operation.fixed_machine_id.as_deref() {
            if !machines
                .iter()
                .any(|machine| machine.machine_id.as_str() == machine_id)
            {
                errors.push(format!(
                    "Machine {} referenced by operation {} does not exist",
                    machine_id, operation.operation_id
                ));
            }
        }
    }

    // Order -> Operation
    for order in orders.iter() {
        if !operations
            .iter()
            .any(|operation| operation.order_id == order.order_id)
        {
            errors.push(format!("Order {} has no operations", order.order_id));
        }
    }

    // Machine -> Company
    for machine in machines.iter() {
        if !companies
            .iter()
            .any(|company| company.company_id == machine.company_id)
        {
            errors.push(format!(
                "Company {} referenced by machine {} does not exist",
                machine.company_id, machine.machine_id
            ));
        }
    }

    // Order -> Company
    for order in orders.iter() {
        if !companies
            .iter()
            .any(|company| company.company_id == order.company_id)
        {
            errors.push(format!(
                "Company {} referenced by order {} does not exist",
                order.company_id, order.order_id
            ));
        }
    }

    errors
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Machine;

    #[test]
    fn valid_machine_should_pass() {
        let machines = vec![Machine {
            machine_id: "M01".to_string(),
            name: "CNC Machine".to_string(),
            available_seconds_per_day: 28800,
            is_active: true,
        }];

        let errors = data_validation(&machines);

        assert!(errors.is_empty());
    }

    #[test]
    fn invalid_machine_should_return_errors() {
        let machines = vec![Machine {
            machine_id: "".to_string(),
            name: "   ".to_string(),
            available_seconds_per_day: -100,
            is_active: true,
        }];

        let errors = data_validation(&machines);

        assert_eq!(errors.len(), 3);
    }

    #[test]
    fn multiple_machines_should_be_validated() {
        let machines = vec![
            Machine {
                machine_id: "M01".to_string(),
                name: "CNC".to_string(),
                available_seconds_per_day: 28800,
                is_active: true,
            },
            Machine {
                machine_id: "M02".to_string(),
                name: "Drilling".to_string(),
                available_seconds_per_day: 0,
                is_active: true,
            },
        ];

        let errors = data_validation(&machines);

        assert_eq!(errors.len(), 1);
    }
}
