use crate::models::{Machine, Operation};

pub fn machine_business_rules(
    machines: &[Machine],
    operations: &[Operation],
) -> Vec<String> {
    let mut business_errors = Vec::new();

    for operation in operations.iter() {
        if let Some(machine_id) = operation.fixed_machine_id.as_deref() {
            if let Some(machine) = machines
                .iter()
                .find(|machine| machine.machine_id.as_str() == machine_id)
            {
                if !machine.is_active {
                    business_errors.push(format!(
                        "Machine {} is inactive but assigned to operation {}",
                        machine.machine_id,
                        operation.operation_id
                    ));
                }
            }
        }
    }

    business_errors
}