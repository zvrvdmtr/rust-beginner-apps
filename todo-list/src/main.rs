mod domain;
mod repository;
mod services;

use domain::entities::{Action, Order};
use services::service::Service;
use std::collections::HashMap;
use std::{ env, process };
use std::error::Error;

use crate::repository::csv_repo::CsvRepo;
use crate::services::errors::ApplicationError;

const DEFAULT_DB: &str = "storage.csv";

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        process::exit(1)
    };
}

fn run() -> Result<(), Box<dyn Error>> {
        let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Not enough arguments".into())
    }

    let storage = CsvRepo::new(DEFAULT_DB);
    let service = Service::new(storage);

    let action = &args[1];
    let params = parse_params(&args[2..]);

    let parsed_action = action.parse::<Action>()?;
    call_action(parsed_action, params, service)
}

fn parse_params(args: &[String]) -> HashMap<String, String> {
    let mut params: HashMap<String, String> = HashMap::new();
    for item in args {
        if let Some((key, value)) = item.split_once("=") {
            params.insert(key.to_string(), value.to_string());
        }
    }
    params
}

fn call_action(
    action: Action,
    params: HashMap<String, String>,
    mut service: Service,
) -> Result<(), Box<dyn Error>> {
    match action {
        Action::Get => {
            let id = params
                .get("id")
                .ok_or_else(|| ApplicationError::InternalError {
                    message: "Task id is required".to_string(),
                })?
                .parse()?;
            let task = service.handle_get(id)?;
            println!("Id: {:?}; Body: {:?};", task.id, task.body);
            Ok(())
        }
        Action::List => {
            let order = params
                .get("order")
                .and_then(|order| order.parse::<Order>().ok());
            let tasks = service.handle_list(order)?;
            for task in tasks.iter() {
                println!("Id: {:?}; Body: {:?};", task.id, task.body);
            }
            Ok(())
        }
        Action::Add => {
            let body = params
                .get("task")
                .ok_or_else(|| ApplicationError::InternalError {
                    message: "Task body is required".to_string(),
                })?;
            service.handle_add(body)?;
            println!("Task created");
            Ok(())
        }
        Action::Delete => {
            let id = params
                .get("id")
                .ok_or_else(|| ApplicationError::InternalError {
                    message: "Task id is required".to_string(),
                })?
                .parse::<u32>()?;
            service.handle_delete(id)?;
            println!("Task deleted");
            Ok(())
        }
    }
}
