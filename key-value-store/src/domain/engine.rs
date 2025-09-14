use std::{collections::HashMap, sync::RwLock};

use crate::{domain::errors::EngineError};

pub trait Manager {
    fn get(&self, key: &str) -> Result<String, EngineError>;
    fn set(&self, key: &str, value: &str) -> Result<(), EngineError>;
    fn delete(&self, key: &str) -> Result<String, EngineError>;
    fn clear(&self) -> Result<(), EngineError>;
    fn total_size(&self) -> Result<usize, EngineError>;
}

pub struct Engine {
    storage: RwLock<HashMap<String, String>>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            storage: RwLock::new(HashMap::new()),
        }
    }
}

impl Manager for Engine {
    fn get(&self, key: &str) -> Result<String, EngineError> {
        let data = self.storage.read()?;
        let value = data.get(key).ok_or(EngineError::NotFound(key.to_string()))?;
        Ok(value.clone())
    }

    fn set(&self, key: &str, value: &str) -> Result<(), EngineError> {
        let mut data = self.storage.write()?;
        data.insert(key.to_string(), value.to_string());
        Ok(())
    }

    fn delete(&self, key: &str) -> Result<String, EngineError> {
        let mut data = self.storage.write()?;
        let value = data.remove(key).ok_or(EngineError::NotFound(key.to_string()))?;
        Ok(value)
    }

    fn clear(&self) -> Result<(), EngineError> {
        let mut data = self.storage.write()?;
        data.clear();
        Ok(())
    }

    fn total_size(&self) -> Result<usize, EngineError> {
        let data = self.storage.read()?;
        Ok(data.len())
    }
}
