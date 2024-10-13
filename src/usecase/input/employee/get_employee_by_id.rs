use crate::entity::model::employee::Employee;
use std::collections::HashMap;

pub fn get_employee_by_id(id: i32, employees: &HashMap<i32, Employee>) -> Option<&Employee> {
    employees.get(&id)
}
