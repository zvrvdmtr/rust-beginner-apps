use crate::{
    domain::entities::{Order, Task},
    repository::traits::Storage,
};

use super::errors::ApplicationError;

pub struct Service {
    repo: Box<dyn Storage>,
}

impl Service {
    pub fn new(repo: impl Storage + 'static) -> Service {
        Service {
            repo: Box::new(repo),
        }
    }

    pub fn handle_get(&self, id: u32) -> Result<Task, ApplicationError> {
        self.repo.get_record_by_id(id).map_err(Into::into)
    }

    pub fn handle_list(&mut self, order: Option<Order>) -> Result<Vec<Task>, ApplicationError> {
        self.repo.list_all_records(order).map_err(Into::into)
    }

    pub fn handle_add(&mut self, body: &str) -> Result<(), ApplicationError> {
        self.repo.add_record(body).map_err(Into::into)
    }

    pub fn handle_delete(&mut self, id: u32) -> Result<(), ApplicationError> {
        self.repo.delete_record_by_id(id).map_err(Into::into)
    }
}
