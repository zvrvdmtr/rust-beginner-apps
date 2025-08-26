use std::{
    fs::{self, File, OpenOptions},
    path::{Path, PathBuf},
};

use csv::{Reader, Writer};

use crate::domain::entities::{Order, Task};

use super::traits::Storage;
use super::errors::RepositoryError;

pub struct CsvRepo {
    file_path: PathBuf,
}

impl CsvRepo {
    pub fn new(file_path: impl Into<PathBuf>) -> CsvRepo {
        CsvRepo {
            file_path: file_path.into(),
        }
    }

    fn writer(&self) -> Result<Writer<File>, RepositoryError> {
        let file_exists = Path::new(&self.file_path).is_file();
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(&self.file_path)
            .map_err(RepositoryError::Io)?;
        let mut writer = Writer::from_writer(file);

        if !file_exists {
            writer.write_record(&["id", "body"])?;
        }
        Ok(writer)
    }

    fn reader(&self) -> Result<Reader<File>, RepositoryError> {
        let reader = Reader::from_path(&self.file_path)?;
        Ok(reader)
    }
}

impl Storage for CsvRepo {
    fn get_record_by_id(&self, id: u32) -> Result<Task, RepositoryError> {
        let mut reader = self.reader()?;

        for record in reader.records() {
            let result = record?;
            let values: Vec<&str> = result.iter().collect();
            if values.len() < 2 {
                continue;
            };
            let id_value: u32 = values[0].parse().map_err(|_e| RepositoryError::InvalidId {
                value: values[0].to_string(),
            })?;

            if id_value == id {
                return Ok(Task {
                    id: id_value,
                    body: values[1].to_string(),
                });
            };
        }

        Err(RepositoryError::NotFound { id: id })
    }

    // TODO: implement order
    fn list_all_records(&self, _: Option<Order>) -> Result<Vec<Task>, RepositoryError> {
        let mut reader = self.reader()?;

        let mut records = vec![];
        for record in reader.records() {
            let result = record?;
            let values: Vec<&str> = result.iter().collect();

            let id_value = values[0].parse().map_err(|_e| RepositoryError::InvalidId {
                value: values[0].to_string(),
            })?;
            records.push(Task {
                id: id_value,
                body: values[1].to_string(),
            });
        }

        Ok(records)
    }

    fn add_record(&mut self, body: &str) -> Result<(), RepositoryError> {
        let mut writer = self.writer()?;
        let mut reader = self.reader()?;

        let mut last_index: u32 = 0;
        for record in reader.records() {
            let result = record?;
            let result_index: u32 = result[0].parse().map_err(|_e| RepositoryError::InvalidId {
                value: result[0].to_string(),
            })?;
            if result_index > last_index {
                last_index = result_index;
            };
        }

        last_index += 1;

        writer.write_record(&[last_index.to_string(), body.to_string()])?;
        writer.flush().map_err(RepositoryError::Io)
    }

    fn delete_record_by_id(&mut self, id: u32) -> Result<(), RepositoryError> {
        let mut reader = self.reader()?;

        let mut tasks: Vec<Task> = vec![];

        for record in reader.records() {
            let result = record?;
            if result[0] == id.to_string() {
                continue;
            }
            tasks.push(Task {
                id: result[0].parse().map_err(|_e| RepositoryError::InvalidId {
                    value: result[0].to_string(),
                })?,
                body: result[1].to_string(),
            });
        }

        fs::remove_file(&self.file_path).map_err(RepositoryError::Io)?;

        let mut writer = self.writer()?;

        for task in tasks {
            writer.write_record(&[task.id.to_string(), task.body])?;
        }

        writer.flush().map_err(RepositoryError::Io)
    }
}
