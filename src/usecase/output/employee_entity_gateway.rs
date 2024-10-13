use crate::entity::model::employee::Employee;

pub trait EmployeeEntityGateway {
    fn save(&mut self, employee: Employee);
    fn get_by_id(&self, id: i32) -> Option<&Employee>;
    fn delete_by_id(&mut self, id: i32) -> Option<Employee>;
}
