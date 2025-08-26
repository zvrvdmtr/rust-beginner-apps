mod domain;
mod services;
mod repository;

use domain::entities::{Action, Order};
use std::collections::HashMap;
use std::env;
use services::service::Service;

use crate::repository::csv_repo::CsvRepo;

const DEFAULT_DB: &str = "storage.csv";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments")
    }

    let storage = CsvRepo::new(DEFAULT_DB);
    let mut service = Service::new(storage);

    let action = &args[1];
    let params = parse_params(&args[2..]);

    match action.parse::<Action>() {
        Ok(action) => match action {
            Action::Get => {
                let id = params
                    .get("id")
                    .expect("Task id is required")
                    .parse::<u32>()
                    .expect("Task id must be integer");
                match service.handle_get(id) {
                    Ok(task) => println!("Id: {:?}; Body: {:?};", task.id, task.body),
                    Err(err) => eprintln!("{:?}", err)
                }
            }
            Action::List => {
                let order = params.get("order").and_then(|order| order.parse::<Order>().ok());
                match service.handle_list(order) {
                    Ok(tasks) => {
                        for task in tasks.iter() {
                            println!("Id: {:?}; Body: {:?};", task.id, task.body);
                        };
                    },
                    Err(err) => eprintln!("{:?}", err)
                };
            }
            Action::Add => {
                let body = params.get("task").expect("Task body is required");
                match service.handle_add(body) {
                    Ok(_) => println!("Task created"),
                    Err(err) => eprintln!("{:?}", err)
                }
            }
            Action::Delete => {
                let id = params
                    .get("id")
                    .expect("Task id is required")
                    .parse::<u32>()
                    .expect("Task id must be integer");
                match service.handle_delete(id) {
                    Ok(_) => println!("Task deleted"),
                    Err(err) => eprintln!("{:?}", err)
                }
            }
        },
        Err(err) => println!("{}", err),
    }
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