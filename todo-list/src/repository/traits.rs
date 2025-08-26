use crate::domain::entities::{Order, Task};
use super::errors::RepositoryError;

pub trait Storage {
    fn get_record_by_id(&self, id: u32) -> Result<Task, RepositoryError>;
    fn list_all_records(&self, order: Option<Order>) -> Result<Vec<Task>, RepositoryError>;
    fn add_record(&mut self, body: &str) -> Result<(), RepositoryError>;
    fn delete_record_by_id(&mut self, id: u32) -> Result<(), RepositoryError>;
}