use crate::{entity::model::employee::Employee, usecase::output::employee_entity_gateway::EmployeeEntityGateway};

pub fn delete_employee(id: i32, gateway: &impl EmployeeEntityGateway) -> Option<Employee> {
    gateway.delete_by_id(&id)
}
