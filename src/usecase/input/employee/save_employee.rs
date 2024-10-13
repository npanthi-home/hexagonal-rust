use crate::entity::model::employee::Employee;
use crate::entity::model::department::Department;
use std::collections::HashMap;

pub fn save_employee(
    id: i32,
    name: String,
    email: String,
    department_id: i32,
    department_name: String,
    employees: &mut HashMap<i32, Employee>,
) -> Employee {
    let department = Department {
        id: department_id,
        name: department_name,
    };

    let employee = Employee {
        id,
        name,
        email,
        department,
    };

    employees.insert(id, employee.clone());
    employee
}
