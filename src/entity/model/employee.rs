use crate::entity::model::department::Department;

#[derive(Debug)]
pub struct Employee {
    pub id: i32,

    pub name: String,

    pub email: String,

    pub department: Department,
}
