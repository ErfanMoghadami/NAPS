use crate::models::{Machine, Operation};

pub fn machine_business_rules(machines: &[Machine], operations: &[Operation]) -> Vec<String> {
    let mut business_errors = Vec::new();

    for operation in operations.iter() {
        for machine in machines.iter() {
            if operation.machine_id == machine.machine_id && !machine.is_active {
                business_errors.push(format!(
                    "machine: {} is not active and operstion: {} used it",
                    operation.machine_id, operation.operation_id
                ));
            }
        }
    }
    business_errors
}
