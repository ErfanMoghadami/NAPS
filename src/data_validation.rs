use crate::models::{Machine, Operation, Order};

pub fn data_validation_machine(machines: &[Machine]) -> Vec<String> {
    let mut errors: Vec<String> = Vec::new();

    for (index, machine) in machines.iter().enumerate() {
        let row = index + 2;

        // Validate machine_id
        if machine.machine_id.trim().is_empty() {
            errors.push(format!("Row {row}: machine_id cannot be empty"));
        }

        // Validate name
        if machine.name.trim().is_empty() {
            errors.push(format!("Row {row}: machine name cannot be empty"));
        }

        // Validate available_seconds_per_day
        if machine.available_seconds_per_day <= 0 {
            errors.push(format!(
                "Row {row}: available_seconds_per_day must be greater than zero"
            ));
        }
    }

    errors
}

pub fn data_validation_orders(orders: &[Order]) -> Vec<String> {
    let mut errors = Vec::new();

    for (index, order) in orders.iter().enumerate() {
        let row = index + 2;

        if order.order_id.trim().is_empty() {
            errors.push(format!("row {row}: order is cant be empty"));
        }
        if order.product.trim().is_empty() {
            errors.push(format!("row {row}: product is cant be empty"));
        }
        if order.quantity <= 0 {
            errors.push(format!("row {row}: quantity cant be zero or under zero"));
        }
        if chrono::DateTime::parse_from_rfc3339(&order.due_at).is_err() {
            errors.push(format!("Row {row}: due_at is not a valid RFC3339 date"));
        }
    }
    errors
}

pub fn data_validation_operations(operations: &[Operation]) -> Vec<String> {
    let mut errors: Vec<String> = Vec::new();

    for (index, operation) in operations.iter().enumerate() {
        let row = index + 2;

        if operation.machine_id.trim().is_empty() {
            errors.push(format!("Row {row}: machine_id cannot be empty"));
        }
        if operation.order_id.trim().is_empty() {
            errors.push(format!("row {row}: order is cant be empty"));
        }
        if operation.operation_id.trim().is_empty() {
            errors.push(format!("row {row} : operation id cant be empty "));
        }
        if operation.processing_time_per_unit_seconds < 0 {
            errors.push(format!("row {row} : setup_time_seconds cannot be negative"));
        }
        if operation.sequence <= 0 {
            errors.push(format!("Row {row}: sequence must be greater than zero"));
        }
        if operation.processing_time_per_unit_seconds <= 0 {
            errors.push(format!(
                "Row {row}: processing_time_per_unit_seconds must be greater than zero"
            ));
        }
    }
    errors
}

pub fn validation_refrence(
    machines: &[Machine],
    orders: &[Order],
    operations: &[Operation],
) -> Vec<String> {
    let mut errors = Vec::new();
    for operation in operations.iter() {
        if !machines
            .iter()
            .any(|machine| machine.machine_id == operation.machine_id)
        {
            errors.push(format!(
                "machine {} is not valid in operation",
                operation.machine_id
            ));
        }
        if !orders
            .iter()
            .any(|order| order.order_id == operation.order_id)
        {
            errors.push(format!(
                "order {} is not valid in operation",
                operation.order_id
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
