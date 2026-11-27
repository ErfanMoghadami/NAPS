use crate::models::{
    CalendarException,
    CalendarExceptionKind,
    Capability,
    Company,
    Machine,
    MachineCalendarAssignment,
    MachineCapability,
    MachineDowntime,
    Operation,
    OperationCapabilityRequirement,
    OperationSchedule,
    Order,
    PlanningRun,
    Product,
    ProductRouting,
    ProductRoutingStep,
    ProductionSite,
    RoutingStepCapabilityRequirement,
    Shift,
    WorkCalendar,
};

use std::collections::HashSet;

// -----------------------------------------------------------------------------
// Shared validation helpers
// -----------------------------------------------------------------------------

fn required(
    value: &str,
    field: &str,
    row: usize,
    errors: &mut Vec<String>,
) {
    if value.trim().is_empty() {
        errors.push(format!(
            "Row {row}: {field} cannot be empty"
        ));
    }
}

fn unique_id(
    value: &str,
    field: &str,
    row: usize,
    seen: &mut HashSet<String>,
    errors: &mut Vec<String>,
) {
    let value = value.trim();

    if value.is_empty() {
        errors.push(format!(
            "Row {row}: {field} cannot be empty"
        ));
    } else if !seen.insert(value.to_owned()) {
        errors.push(format!(
            "Row {row}: duplicate {field}: {value}"
        ));
    }
}

fn non_negative(
    value: i64,
    field: &str,
    row: usize,
    errors: &mut Vec<String>,
) {
    if value < 0 {
        errors.push(format!(
            "Row {row}: {field} cannot be negative"
        ));
    }
}

fn positive(
    value: i64,
    field: &str,
    row: usize,
    errors: &mut Vec<String>,
) {
    if value <= 0 {
        errors.push(format!(
            "Row {row}: {field} must be greater than zero"
        ));
    }
}

fn date_ranges_overlap(
    first_start: Option<chrono::NaiveDate>,
    first_end: Option<chrono::NaiveDate>,
    second_start: Option<chrono::NaiveDate>,
    second_end: Option<chrono::NaiveDate>,
) -> bool {
    let first_starts_before_second_ends =
        match (first_start, second_end) {
            (Some(start), Some(end)) => start <= end,
            _ => true,
        };

    let second_starts_before_first_ends =
        match (second_start, first_end) {
            (Some(start), Some(end)) => start <= end,
            _ => true,
        };

    first_starts_before_second_ends
        && second_starts_before_first_ends
}

// -----------------------------------------------------------------------------
// Company and production location
// -----------------------------------------------------------------------------

pub fn data_validation_companies(
    companies: &[Company],
) -> Vec<String> {
    let mut errors = Vec::new();
    let mut seen_ids = HashSet::new();

    for (index, company) in companies.iter().enumerate() {
        let row = index + 2;

        unique_id(
            &company.company_id,
            "company_id",
            row,
            &mut seen_ids,
            &mut errors,
        );

        required(&company.code, "code", row, &mut errors);
        required(&company.name, "name", row, &mut errors);
    }

    errors
}

pub fn data_validation_production_sites(
    sites: &[ProductionSite],
) -> Vec<String> {
    let mut errors = Vec::new();
    let mut seen_ids = HashSet::new();

    for (index, site) in sites.iter().enumerate() {
        let row = index + 2;

        unique_id(
            &site.production_site_id,
            "production_site_id",
            row,
            &mut seen_ids,
            &mut errors,
        );

        required(&site.company_id, "company_id", row, &mut errors);
        required(&site.code, "code", row, &mut errors);
        required(&site.name, "name", row, &mut errors);
        required(&site.timezone, "timezone", row, &mut errors);
    }

    errors
}

// -----------------------------------------------------------------------------
// Machines and capabilities
// -----------------------------------------------------------------------------

pub fn data_validation_machine(
    machines: &[Machine],
) -> Vec<String> {
    let mut errors = Vec::new();
    let mut seen_ids = HashSet::new();

    for (index, machine) in machines.iter().enumerate() {
        let row = index + 2;

        unique_id(
            &machine.machine_id,
            "machine_id",
            row,
            &mut seen_ids,
            &mut errors,
        );

        required(&machine.company_id, "company_id", row, &mut errors);
        required(
            &machine.production_site_id,
            "production_site_id",
            row,
            &mut errors,
        );
        required(&machine.code, "code", row, &mut errors);
        required(&machine.name, "name", row, &mut errors);
    }

    errors
}

pub fn data_validation_capabilities(
    capabilities: &[Capability],
) -> Vec<String> {
    let mut errors = Vec::new();
    let mut seen_ids = HashSet::new();

    for (index, capability) in capabilities.iter().enumerate() {
        let row = index + 2;

        unique_id(
            &capability.capability_id,
            "capability_id",
            row,
            &mut seen_ids,
            &mut errors,
        );

        required(
            &capability.company_id,
            "company_id",
            row,
            &mut errors,
        );
        required(&capability.code, "code", row, &mut errors);
        required(&capability.name, "name", row, &mut errors);
    }

    errors
}

pub fn data_validation_machine_capabilities(
    machine_capabilities: &[MachineCapability],
) -> Vec<String> {
    let mut errors = Vec::new();
    let mut seen_ids = HashSet::new();

    for (index, item) in machine_capabilities.iter().enumerate() {
        let row = index + 2;

        unique_id(
            &item.machine_capability_id,
            "machine_capability_id",
            row,
            &mut seen_ids,
            &mut errors,
        );

        required(&item.machine_id, "machine_id", row, &mut errors);
        required(&item.capability_id, "capability_id", row, &mut errors);

        if let (Some(valid_from), Some(valid_until)) =
            (item.valid_from, item.valid_until)
        {
            if valid_until < valid_from {
                errors.push(format!(
                    "Row {row}: valid_until cannot be earlier than valid_from"
                ));
            }
        }
    }

    // The model explicitly permits multiple qualifications for the same
    // machine/capability pair only when their validity periods do not overlap.
    for first_index in 0..machine_capabilities.len() {
        for second_index in (first_index + 1)..machine_capabilities.len() {
            let first = &machine_capabilities[first_index];
            let second = &machine_capabilities[second_index];

            let same_pair =
                first.machine_id == second.machine_id
                && first.capability_id == second.capability_id;

            if same_pair
                && date_ranges_overlap(
                    first.valid_from,
                    first.valid_until,
                    second.valid_from,
                    second.valid_until,
                )
            {
                errors.push(format!(
                    "Rows {} and {}: validity periods overlap for machine {} and capability {}",
                    first_index + 2,
                    second_index + 2,
                    first.machine_id,
                    first.capability_id
                ));
            }
        }
    }

    errors
}

// -----------------------------------------------------------------------------
// Products and reusable production routings
// -----------------------------------------------------------------------------

pub fn data_validation_products(
    products: &[Product],
) -> Vec<String> {
    let mut errors = Vec::new();
    let mut seen_ids = HashSet::new();

    for (index, product) in products.iter().enumerate() {
        let row = index + 2;

        unique_id(
            &product.product_id,
            "product_id",
            row,
            &mut seen_ids,
            &mut errors,
        );

        required(&product.company_id, "company_id", row, &mut errors);
        required(&product.code, "code", row, &mut errors);
        required(&product.name, "name", row, &mut errors);

        required(
            &product.base_unit_of_measure,
            "base_unit_of_measure",
            row,
            &mut errors,
        );
    }

    errors
}

pub fn data_validation_product_routings(
    routings: &[ProductRouting],
) -> Vec<String> {
    let mut errors = Vec::new();
    let mut seen_ids = HashSet::new();

    for (index, routing) in routings.iter().enumerate() {
        let row = index + 2;

        unique_id(
            &routing.routing_id,
            "routing_id",
            row,
            &mut seen_ids,
            &mut errors,
        );

        required(&routing.product_id, "product_id", row, &mut errors);
        required(&routing.name, "name", row, &mut errors);

        if routing.version == 0 {
            errors.push(format!(
                "Row {row}: version must be greater than zero"
            ));
        }
    }

    errors
}

pub fn data_validation_product_routing_steps(
    steps: &[ProductRoutingStep],
) -> Vec<String> {
    let mut errors = Vec::new();
    let mut seen_ids = HashSet::new();
    let mut seen_sequences = HashSet::new();

    for (index, step) in steps.iter().enumerate() {
        let row = index + 2;

        unique_id(
            &step.routing_step_id,
            "routing_step_id",
            row,
            &mut seen_ids,
            &mut errors,
        );

        required(&step.routing_id, "routing_id", row, &mut errors);
        required(&step.name, "name", row, &mut errors);

        if step.sequence == 0 {
            errors.push(format!(
                "Row {row}: sequence must be greater than zero"
            ));
        } else if !seen_sequences.insert((
            step.routing_id.trim().to_owned(),
            step.sequence,
        )) {
            errors.push(format!(
                "Row {row}: duplicate sequence {} in routing {}",
                step.sequence,
                step.routing_id
            ));
        }

        non_negative(
            step.setup_time_seconds,
            "setup_time_seconds",
            row,
            &mut errors,
        );

        positive(
            step.processing_time_per_unit_seconds,
            "processing_time_per_unit_seconds",
            row,
            &mut errors,
        );
    }

    errors
}

pub fn data_validation_routing_step_capability_requirements(
    requirements: &[RoutingStepCapabilityRequirement],
) -> Vec<String> {
    let mut errors = Vec::new();
    let mut seen_pairs = HashSet::new();

    for (index, requirement) in requirements.iter().enumerate() {
        let row = index + 2;

        required(
            &requirement.routing_step_id,
            "routing_step_id",
            row,
            &mut errors,
        );

        required(
            &requirement.capability_id,
            "capability_id",
            row,
            &mut errors,
        );

        let pair = (
            requirement.routing_step_id.trim().to_owned(),
            requirement.capability_id.trim().to_owned(),
        );

        if !pair.0.is_empty()
            && !pair.1.is_empty()
            && !seen_pairs.insert(pair)
        {
            errors.push(format!(
                "Row {row}: duplicate capability requirement for routing step {} and capability {}",
                requirement.routing_step_id,
                requirement.capability_id
            ));
        }
    }

    errors
}

// -----------------------------------------------------------------------------
// Production orders and operation snapshots
// -----------------------------------------------------------------------------

pub fn data_validation_orders(
    orders: &[Order],
) -> Vec<String> {
    let mut errors = Vec::new();
    let mut seen_ids = HashSet::new();

    for (index, order) in orders.iter().enumerate() {
        let row = index + 2;

        unique_id(
            &order.order_id,
            "order_id",
            row,
            &mut seen_ids,
            &mut errors,
        );

        required(&order.company_id, "company_id", row, &mut errors);
        required(&order.product_id, "product_id", row, &mut errors);
        required(&order.routing_id, "routing_id", row, &mut errors);

        if order.quantity <= 0 {
            errors.push(format!(
                "Row {row}: quantity must be greater than zero"
            ));
        }

        if let Some(released_at) = order.released_at.as_ref() {
            if released_at < &order.created_at {
                errors.push(format!(
                    "Row {row}: released_at cannot be earlier than created_at"
                ));
            }
        }
    }

    errors
}

pub fn data_validation_operations(
    operations: &[Operation],
) -> Vec<String> {
    let mut errors = Vec::new();
    let mut seen_ids = HashSet::new();
    let mut seen_sequences = HashSet::new();

    for (index, operation) in operations.iter().enumerate() {
        let row = index + 2;

        unique_id(
            &operation.operation_id,
            "operation_id",
            row,
            &mut seen_ids,
            &mut errors,
        );

        required(&operation.order_id, "order_id", row, &mut errors);

        required(
            &operation.routing_step_id,
            "routing_step_id",
            row,
            &mut errors,
        );

        if operation.sequence == 0 {
            errors.push(format!(
                "Row {row}: sequence must be greater than zero"
            ));
        } else if !seen_sequences.insert((
            operation.order_id.trim().to_owned(),
            operation.sequence,
        )) {
            errors.push(format!(
                "Row {row}: duplicate sequence {} in order {}",
                operation.sequence,
                operation.order_id
            ));
        }

        if let Some(machine_id) = operation.fixed_machine_id.as_deref() {
            required(
                machine_id,
                "fixed_machine_id",
                row,
                &mut errors,
            );
        }

        non_negative(
            operation.setup_time_seconds,
            "setup_time_seconds",
            row,
            &mut errors,
        );

        positive(
            operation.processing_time_per_unit_seconds,
            "processing_time_per_unit_seconds",
            row,
            &mut errors,
        );
    }

    errors
}

pub fn data_validation_operation_capability_requirements(
    requirements: &[OperationCapabilityRequirement],
) -> Vec<String> {
    let mut errors = Vec::new();
    let mut seen_pairs = HashSet::new();

    for (index, requirement) in requirements.iter().enumerate() {
        let row = index + 2;

        required(
            &requirement.operation_id,
            "operation_id",
            row,
            &mut errors,
        );

        required(
            &requirement.capability_id,
            "capability_id",
            row,
            &mut errors,
        );

        let pair = (
            requirement.operation_id.trim().to_owned(),
            requirement.capability_id.trim().to_owned(),
        );

        if !pair.0.is_empty()
            && !pair.1.is_empty()
            && !seen_pairs.insert(pair)
        {
            errors.push(format!(
                "Row {row}: duplicate capability requirement for operation {} and capability {}",
                requirement.operation_id,
                requirement.capability_id
            ));
        }
    }

    errors
}

// -----------------------------------------------------------------------------
// Work calendars and machine availability
// -----------------------------------------------------------------------------

pub fn data_validation_work_calendars(
    calendars: &[WorkCalendar],
) -> Vec<String> {
    let mut errors = Vec::new();
    let mut seen_ids = HashSet::new();

    for (index, calendar) in calendars.iter().enumerate() {
        let row = index + 2;

        unique_id(
            &calendar.calendar_id,
            "calendar_id",
            row,
            &mut seen_ids,
            &mut errors,
        );

        required(
            &calendar.production_site_id,
            "production_site_id",
            row,
            &mut errors,
        );

        required(&calendar.name, "name", row, &mut errors);
    }

    errors
}

pub fn data_validation_shifts(
    shifts: &[Shift],
) -> Vec<String> {
    let mut errors = Vec::new();
    let mut seen_ids = HashSet::new();

    for (index, shift) in shifts.iter().enumerate() {
        let row = index + 2;

        unique_id(
            &shift.shift_id,
            "shift_id",
            row,
            &mut seen_ids,
            &mut errors,
        );

        required(&shift.calendar_id, "calendar_id", row, &mut errors);
        required(&shift.name, "name", row, &mut errors);

        if shift.days_of_week.is_empty() {
            errors.push(format!(
                "Row {row}: days_of_week cannot be empty"
            ));
        }

        let mut seen_days = HashSet::new();

        for day in &shift.days_of_week {
            if !(1..=7).contains(day) {
                errors.push(format!(
                    "Row {row}: invalid day {day}; days_of_week must contain values from 1 to 7"
                ));
            } else if !seen_days.insert(*day) {
                errors.push(format!(
                    "Row {row}: duplicate day {day} in days_of_week"
                ));
            }
        }

        if shift.start_time == shift.end_time {
            errors.push(format!(
                "Row {row}: start_time and end_time cannot be equal"
            ));
        }
    }

    errors
}

pub fn data_validation_calendar_exceptions(
    exceptions: &[CalendarException],
) -> Vec<String> {
    let mut errors = Vec::new();
    let mut seen_ids = HashSet::new();
    let mut seen_calendar_dates = HashSet::new();

    for (index, exception) in exceptions.iter().enumerate() {
        let row = index + 2;

        unique_id(
            &exception.calendar_exception_id,
            "calendar_exception_id",
            row,
            &mut seen_ids,
            &mut errors,
        );

        required(
            &exception.calendar_id,
            "calendar_id",
            row,
            &mut errors,
        );

        let calendar_date = (
            exception.calendar_id.trim().to_owned(),
            exception.date,
        );

        if !calendar_date.0.is_empty()
            && !seen_calendar_dates.insert(calendar_date)
        {
            errors.push(format!(
                "Row {row}: more than one exception is defined for calendar {} on {}",
                exception.calendar_id,
                exception.date
            ));
        }

        match (exception.start_time, exception.end_time) {
            (Some(start), Some(end)) if start == end => {
                errors.push(format!(
                    "Row {row}: exception start_time and end_time cannot be equal"
                ));
            }

            (Some(_), None) | (None, Some(_)) => {
                errors.push(format!(
                    "Row {row}: start_time and end_time must either both be provided or both be empty"
                ));
            }

            _ => {}
        }

        if exception.kind == CalendarExceptionKind::ModifiedWorkingHours
            && (
                exception.start_time.is_none()
                    || exception.end_time.is_none()
            )
        {
            errors.push(format!(
                "Row {row}: modified_working_hours requires start_time and end_time"
            ));
        }
    }

    errors
}

pub fn data_validation_machine_calendar_assignments(
    assignments: &[MachineCalendarAssignment],
) -> Vec<String> {
    let mut errors = Vec::new();
    let mut seen_ids = HashSet::new();

    for (index, assignment) in assignments.iter().enumerate() {
        let row = index + 2;

        unique_id(
            &assignment.assignment_id,
            "assignment_id",
            row,
            &mut seen_ids,
            &mut errors,
        );

        required(
            &assignment.machine_id,
            "machine_id",
            row,
            &mut errors,
        );

        required(
            &assignment.calendar_id,
            "calendar_id",
            row,
            &mut errors,
        );

        if let Some(valid_until) = assignment.valid_until {
            if valid_until < assignment.valid_from {
                errors.push(format!(
                    "Row {row}: valid_until cannot be earlier than valid_from"
                ));
            }
        }
    }

    errors
}

pub fn data_validation_machine_downtimes(
    downtimes: &[MachineDowntime],
) -> Vec<String> {
    let mut errors = Vec::new();
    let mut seen_ids = HashSet::new();

    for (index, downtime) in downtimes.iter().enumerate() {
        let row = index + 2;

        unique_id(
            &downtime.downtime_id,
            "downtime_id",
            row,
            &mut seen_ids,
            &mut errors,
        );

        required(
            &downtime.machine_id,
            "machine_id",
            row,
            &mut errors,
        );

        required(&downtime.reason, "reason", row, &mut errors);

        if downtime.end_at <= downtime.start_at {
            errors.push(format!(
                "Row {row}: end_at must be later than start_at"
            ));
        }
    }

    errors
}

// -----------------------------------------------------------------------------
// APS result models
// -----------------------------------------------------------------------------
//
// These are not initial input CSV datasets. They are validated when the APS
// engine creates them or when persisted planning results are loaded later.

pub fn data_validation_planning_runs(
    planning_runs: &[PlanningRun],
) -> Vec<String> {
    let mut errors = Vec::new();
    let mut seen_ids = HashSet::new();

    for (index, run) in planning_runs.iter().enumerate() {
        let row = index + 2;

        unique_id(
            &run.planning_run_id,
            "planning_run_id",
            row,
            &mut seen_ids,
            &mut errors,
        );

        required(&run.company_id, "company_id", row, &mut errors);

        required(
            &run.production_site_id,
            "production_site_id",
            row,
            &mut errors,
        );

        required(
            &run.algorithm_version,
            "algorithm_version",
            row,
            &mut errors,
        );

        if run.horizon_end <= run.horizon_start {
            errors.push(format!(
                "Row {row}: horizon_end must be later than horizon_start"
            ));
        }

        if let Some(started_at) = run.started_at.as_ref() {
            if started_at < &run.created_at {
                errors.push(format!(
                    "Row {row}: started_at cannot be earlier than created_at"
                ));
            }
        }

        if let Some(completed_at) = run.completed_at.as_ref() {
            if completed_at < &run.created_at {
                errors.push(format!(
                    "Row {row}: completed_at cannot be earlier than created_at"
                ));
            }

            if let Some(started_at) = run.started_at.as_ref() {
                if completed_at < started_at {
                    errors.push(format!(
                        "Row {row}: completed_at cannot be earlier than started_at"
                    ));
                }
            }
        }
    }

    errors
}

pub fn data_validation_operation_schedules(
    schedules: &[OperationSchedule],
) -> Vec<String> {
    let mut errors = Vec::new();
    let mut seen_ids = HashSet::new();

    for (index, schedule) in schedules.iter().enumerate() {
        let row = index + 2;

        unique_id(
            &schedule.operation_schedule_id,
            "operation_schedule_id",
            row,
            &mut seen_ids,
            &mut errors,
        );

        required(
            &schedule.planning_run_id,
            "planning_run_id",
            row,
            &mut errors,
        );

        required(
            &schedule.operation_id,
            "operation_id",
            row,
            &mut errors,
        );

        required(
            &schedule.machine_id,
            "machine_id",
            row,
            &mut errors,
        );

        if schedule.planned_end_at <= schedule.planned_start_at {
            errors.push(format!(
                "Row {row}: planned_end_at must be later than planned_start_at"
            ));
        }
    }

    errors
}

// -----------------------------------------------------------------------------
// Reference validation for the 17 input datasets
// -----------------------------------------------------------------------------

pub fn validation_reference(
    companies: &[Company],
    production_sites: &[ProductionSite],
    machines: &[Machine],
    capabilities: &[Capability],
    machine_capabilities: &[MachineCapability],
    products: &[Product],
    product_routings: &[ProductRouting],
    product_routing_steps: &[ProductRoutingStep],
    routing_step_capability_requirements: &[RoutingStepCapabilityRequirement],
    orders: &[Order],
    operations: &[Operation],
    operation_capability_requirements: &[OperationCapabilityRequirement],
    work_calendars: &[WorkCalendar],
    shifts: &[Shift],
    calendar_exceptions: &[CalendarException],
    machine_calendar_assignments: &[MachineCalendarAssignment],
    machine_downtimes: &[MachineDowntime],
) -> Vec<String> {
    let mut errors = Vec::new();

    // ProductionSite -> Company
    for site in production_sites {
        if !companies
            .iter()
            .any(|company| company.company_id == site.company_id)
        {
            errors.push(format!(
                "Company {} referenced by production site {} does not exist",
                site.company_id,
                site.production_site_id
            ));
        }
    }

    // Machine -> Company and ProductionSite
    for machine in machines {
        if !companies
            .iter()
            .any(|company| company.company_id == machine.company_id)
        {
            errors.push(format!(
                "Company {} referenced by machine {} does not exist",
                machine.company_id,
                machine.machine_id
            ));
        }

        match production_sites
            .iter()
            .find(|site| site.production_site_id == machine.production_site_id)
        {
            None => {
                errors.push(format!(
                    "Production site {} referenced by machine {} does not exist",
                    machine.production_site_id,
                    machine.machine_id
                ));
            }

            Some(site) if site.company_id != machine.company_id => {
                errors.push(format!(
                    "Machine {} and production site {} belong to different companies",
                    machine.machine_id,
                    site.production_site_id
                ));
            }

            Some(_) => {}
        }
    }

    // Capability -> Company
    for capability in capabilities {
        if !companies
            .iter()
            .any(|company| company.company_id == capability.company_id)
        {
            errors.push(format!(
                "Company {} referenced by capability {} does not exist",
                capability.company_id,
                capability.capability_id
            ));
        }
    }

    // MachineCapability -> Machine and Capability
    for item in machine_capabilities {
        let machine = machines
            .iter()
            .find(|machine| machine.machine_id == item.machine_id);

        let capability = capabilities
            .iter()
            .find(|capability| capability.capability_id == item.capability_id);

        if machine.is_none() {
            errors.push(format!(
                "Machine {} referenced by machine capability {} does not exist",
                item.machine_id,
                item.machine_capability_id
            ));
        }

        if capability.is_none() {
            errors.push(format!(
                "Capability {} referenced by machine capability {} does not exist",
                item.capability_id,
                item.machine_capability_id
            ));
        }

        if let (Some(machine), Some(capability)) = (machine, capability) {
            if machine.company_id != capability.company_id {
                errors.push(format!(
                    "Machine {} and capability {} belong to different companies",
                    machine.machine_id,
                    capability.capability_id
                ));
            }
        }
    }

    // Product -> Company
    for product in products {
        if !companies
            .iter()
            .any(|company| company.company_id == product.company_id)
        {
            errors.push(format!(
                "Company {} referenced by product {} does not exist",
                product.company_id,
                product.product_id
            ));
        }
    }

    // ProductRouting -> Product
    for routing in product_routings {
        if !products
            .iter()
            .any(|product| product.product_id == routing.product_id)
        {
            errors.push(format!(
                "Product {} referenced by routing {} does not exist",
                routing.product_id,
                routing.routing_id
            ));
        }
    }

    // ProductRoutingStep -> ProductRouting
    for step in product_routing_steps {
        if !product_routings
            .iter()
            .any(|routing| routing.routing_id == step.routing_id)
        {
            errors.push(format!(
                "Routing {} referenced by routing step {} does not exist",
                step.routing_id,
                step.routing_step_id
            ));
        }
    }

    // RoutingStepCapabilityRequirement -> RoutingStep and Capability
    for requirement in routing_step_capability_requirements {
        let step = product_routing_steps
            .iter()
            .find(|step| step.routing_step_id == requirement.routing_step_id);

        let capability = capabilities
            .iter()
            .find(|capability| capability.capability_id == requirement.capability_id);

        if step.is_none() {
            errors.push(format!(
                "Routing step {} referenced by capability requirement does not exist",
                requirement.routing_step_id
            ));
        }

        if capability.is_none() {
            errors.push(format!(
                "Capability {} referenced by routing step {} does not exist",
                requirement.capability_id,
                requirement.routing_step_id
            ));
        }

        if let (Some(step), Some(capability)) = (step, capability) {
            if let Some(routing) = product_routings
                .iter()
                .find(|routing| routing.routing_id == step.routing_id)
            {
                if let Some(product) = products
                    .iter()
                    .find(|product| product.product_id == routing.product_id)
                {
                    if product.company_id != capability.company_id {
                        errors.push(format!(
                            "Capability {} and routing step {} belong to different companies",
                            capability.capability_id,
                            step.routing_step_id
                        ));
                    }
                }
            }
        }
    }

    // Order -> Company, Product and ProductRouting.
    // The selected routing must belong to the same product as the order.
    for order in orders {
        if !companies
            .iter()
            .any(|company| company.company_id == order.company_id)
        {
            errors.push(format!(
                "Company {} referenced by order {} does not exist",
                order.company_id,
                order.order_id
            ));
        }

        match products
            .iter()
            .find(|product| product.product_id == order.product_id)
        {
            None => {
                errors.push(format!(
                    "Product {} referenced by order {} does not exist",
                    order.product_id,
                    order.order_id
                ));
            }

            Some(product) if product.company_id != order.company_id => {
                errors.push(format!(
                    "Product {} and order {} belong to different companies",
                    product.product_id,
                    order.order_id
                ));
            }

            Some(_) => {}
        }

        match product_routings
            .iter()
            .find(|routing| routing.routing_id == order.routing_id)
        {
            None => {
                errors.push(format!(
                    "Routing {} referenced by order {} does not exist",
                    order.routing_id,
                    order.order_id
                ));
            }

            Some(routing) if routing.product_id != order.product_id => {
                errors.push(format!(
                    "Routing {} does not belong to product {} on order {}",
                    routing.routing_id,
                    order.product_id,
                    order.order_id
                ));
            }

            Some(_) => {}
        }
    }

    // Operation -> Order, RoutingStep and optionally FixedMachine.
    for operation in operations {
        let order = orders
            .iter()
            .find(|order| order.order_id == operation.order_id);

        let step = product_routing_steps
            .iter()
            .find(|step| step.routing_step_id == operation.routing_step_id);

        if order.is_none() {
            errors.push(format!(
                "Order {} referenced by operation {} does not exist",
                operation.order_id,
                operation.operation_id
            ));
        }

        if step.is_none() {
            errors.push(format!(
                "Routing step {} referenced by operation {} does not exist",
                operation.routing_step_id,
                operation.operation_id
            ));
        }

        if let (Some(order), Some(step)) = (order, step) {
            if order.routing_id != step.routing_id {
                errors.push(format!(
                    "Operation {} uses routing step {} from a different routing than order {}",
                    operation.operation_id,
                    step.routing_step_id,
                    order.order_id
                ));
            }
        }

        if let Some(machine_id) = operation.fixed_machine_id.as_deref() {
            match machines
                .iter()
                .find(|machine| machine.machine_id == machine_id)
            {
                None => {
                    errors.push(format!(
                        "Machine {} referenced by operation {} does not exist",
                        machine_id,
                        operation.operation_id
                    ));
                }

                Some(machine) => {
                    if let Some(order) = order {
                        if machine.company_id != order.company_id {
                            errors.push(format!(
                                "Machine {} and order {} belong to different companies",
                                machine.machine_id,
                                order.order_id
                            ));
                        }
                    }
                }
            }
        }
    }

    // OperationCapabilityRequirement -> Operation and Capability
    for requirement in operation_capability_requirements {
        let operation = operations
            .iter()
            .find(|operation| operation.operation_id == requirement.operation_id);

        let capability = capabilities
            .iter()
            .find(|capability| capability.capability_id == requirement.capability_id);

        if operation.is_none() {
            errors.push(format!(
                "Operation {} referenced by capability requirement does not exist",
                requirement.operation_id
            ));
        }

        if capability.is_none() {
            errors.push(format!(
                "Capability {} referenced by operation {} does not exist",
                requirement.capability_id,
                requirement.operation_id
            ));
        }

        if let (Some(operation), Some(capability)) = (operation, capability) {
            if let Some(order) = orders
                .iter()
                .find(|order| order.order_id == operation.order_id)
            {
                if order.company_id != capability.company_id {
                    errors.push(format!(
                        "Capability {} and operation {} belong to different companies",
                        capability.capability_id,
                        operation.operation_id
                    ));
                }
            }
        }
    }

    // WorkCalendar -> ProductionSite
    for calendar in work_calendars {
        if !production_sites
            .iter()
            .any(|site| site.production_site_id == calendar.production_site_id)
        {
            errors.push(format!(
                "Production site {} referenced by calendar {} does not exist",
                calendar.production_site_id,
                calendar.calendar_id
            ));
        }
    }

    // Shift -> WorkCalendar
    for shift in shifts {
        if !work_calendars
            .iter()
            .any(|calendar| calendar.calendar_id == shift.calendar_id)
        {
            errors.push(format!(
                "Calendar {} referenced by shift {} does not exist",
                shift.calendar_id,
                shift.shift_id
            ));
        }
    }

    // CalendarException -> WorkCalendar
    for exception in calendar_exceptions {
        if !work_calendars
            .iter()
            .any(|calendar| calendar.calendar_id == exception.calendar_id)
        {
            errors.push(format!(
                "Calendar {} referenced by exception {} does not exist",
                exception.calendar_id,
                exception.calendar_exception_id
            ));
        }
    }

    // MachineCalendarAssignment -> Machine and WorkCalendar.
    // Both must refer to the same production site.
    for assignment in machine_calendar_assignments {
        let machine = machines
            .iter()
            .find(|machine| machine.machine_id == assignment.machine_id);

        let calendar = work_calendars
            .iter()
            .find(|calendar| calendar.calendar_id == assignment.calendar_id);

        if machine.is_none() {
            errors.push(format!(
                "Machine {} referenced by calendar assignment {} does not exist",
                assignment.machine_id,
                assignment.assignment_id
            ));
        }

        if calendar.is_none() {
            errors.push(format!(
                "Calendar {} referenced by machine assignment {} does not exist",
                assignment.calendar_id,
                assignment.assignment_id
            ));
        }

        if let (Some(machine), Some(calendar)) = (machine, calendar) {
            if machine.production_site_id != calendar.production_site_id {
                errors.push(format!(
                    "Machine {} and calendar {} belong to different production sites",
                    machine.machine_id,
                    calendar.calendar_id
                ));
            }
        }
    }

    // MachineDowntime -> Machine
    for downtime in machine_downtimes {
        if !machines
            .iter()
            .any(|machine| machine.machine_id == downtime.machine_id)
        {
            errors.push(format!(
                "Machine {} referenced by downtime {} does not exist",
                downtime.machine_id,
                downtime.downtime_id
            ));
        }
    }

    errors
}

// -----------------------------------------------------------------------------
// References for generated APS results
// -----------------------------------------------------------------------------
//
// These checks will be used after the APS engine starts creating planning runs
// and operation schedules.

pub fn validation_planning_result_references(
    planning_runs: &[PlanningRun],
    schedules: &[OperationSchedule],
    companies: &[Company],
    production_sites: &[ProductionSite],
    orders: &[Order],
    operations: &[Operation],
    machines: &[Machine],
) -> Vec<String> {
    let mut errors = Vec::new();

    // PlanningRun -> Company and ProductionSite
    for run in planning_runs {
        if !companies
            .iter()
            .any(|company| company.company_id == run.company_id)
        {
            errors.push(format!(
                "Company {} referenced by planning run {} does not exist",
                run.company_id,
                run.planning_run_id
            ));
        }

        match production_sites
            .iter()
            .find(|site| site.production_site_id == run.production_site_id)
        {
            None => {
                errors.push(format!(
                    "Production site {} referenced by planning run {} does not exist",
                    run.production_site_id,
                    run.planning_run_id
                ));
            }

            Some(site) if site.company_id != run.company_id => {
                errors.push(format!(
                    "Planning run {} and production site {} belong to different companies",
                    run.planning_run_id,
                    site.production_site_id
                ));
            }

            Some(_) => {}
        }
    }

    // OperationSchedule -> PlanningRun, Operation and Machine
    for schedule in schedules {
        let run = planning_runs
            .iter()
            .find(|run| run.planning_run_id == schedule.planning_run_id);

        let operation = operations
            .iter()
            .find(|operation| operation.operation_id == schedule.operation_id);

        let machine = machines
            .iter()
            .find(|machine| machine.machine_id == schedule.machine_id);

        if run.is_none() {
            errors.push(format!(
                "Planning run {} referenced by schedule {} does not exist",
                schedule.planning_run_id,
                schedule.operation_schedule_id
            ));
        }

        if operation.is_none() {
            errors.push(format!(
                "Operation {} referenced by schedule {} does not exist",
                schedule.operation_id,
                schedule.operation_schedule_id
            ));
        }

        if machine.is_none() {
            errors.push(format!(
                "Machine {} referenced by schedule {} does not exist",
                schedule.machine_id,
                schedule.operation_schedule_id
            ));
        }

        if let (Some(run), Some(machine)) = (run, machine) {
            if run.production_site_id != machine.production_site_id {
                errors.push(format!(
                    "Machine {} is outside the production site of planning run {}",
                    machine.machine_id,
                    run.planning_run_id
                ));
            }

            if run.company_id != machine.company_id {
                errors.push(format!(
                    "Machine {} and planning run {} belong to different companies",
                    machine.machine_id,
                    run.planning_run_id
                ));
            }
        }

        if let (Some(run), Some(operation)) = (run, operation) {
            if let Some(order) = orders
                .iter()
                .find(|order| order.order_id == operation.order_id)
            {
                if run.company_id != order.company_id {
                    errors.push(format!(
                        "Operation {} and planning run {} belong to different companies",
                        operation.operation_id,
                        run.planning_run_id
                    ));
                }
            }
        }
    }

    errors
}

// -----------------------------------------------------------------------------
// Unit tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_company_should_pass() {
        let companies = vec![Company {
            company_id: "C001".to_string(),
            code: "ALPHA".to_string(),
            name: "Alpha Manufacturing".to_string(),
            is_active: true,
        }];

        assert!(data_validation_companies(&companies).is_empty());
    }

    #[test]
    fn duplicate_company_id_should_return_error() {
        let companies = vec![
            Company {
                company_id: "C001".to_string(),
                code: "ALPHA".to_string(),
                name: "Alpha".to_string(),
                is_active: true,
            },
            Company {
                company_id: "C001".to_string(),
                code: "BETA".to_string(),
                name: "Beta".to_string(),
                is_active: true,
            },
        ];

        let errors = data_validation_companies(&companies);

        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("duplicate company_id"));
    }

    #[test]
    fn invalid_machine_should_return_errors() {
        let machines = vec![Machine {
            machine_id: "".to_string(),
            company_id: "C001".to_string(),
            production_site_id: "PS001".to_string(),
            code: "   ".to_string(),
            name: "   ".to_string(),
            description: None,
            is_active: true,
        }];

        let errors = data_validation_machine(&machines);

        assert_eq!(errors.len(), 3);
    }
}