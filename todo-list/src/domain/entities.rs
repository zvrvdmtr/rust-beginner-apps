use std::str::FromStr;

#[derive(Debug)]
pub struct Task {
    pub id: u32,
    pub body: String,
}

#[derive(Debug)]
pub enum Order {
    Asc,
    Desc,
}

impl FromStr for Order {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "asc" => Ok(Order::Asc),
            "desc" => Ok(Order::Desc),
            _ => Err("Invalid order value. Available values is: \"asc\", \"desc\"".to_string()),
        }
    }
}

#[derive(Debug)]
pub enum Action {
    Get,
    List,
    Add,
    Delete,
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "get" => Ok(Action::Get),
            "list" => Ok(Action::List),
            "add" => Ok(Action::Add),
            "delete" => Ok(Action::Delete),
            _ => Err(
                "Invalid action value. Please use one of: \"get\", \"list\", \"add\" or \"delete\""
                    .to_string(),
            ),
        }
    }
}
