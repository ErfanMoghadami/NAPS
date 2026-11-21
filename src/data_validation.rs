use crate::models::Machine;

pub fn data_validation(machines: &[Machine]) -> Vec<String> {
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