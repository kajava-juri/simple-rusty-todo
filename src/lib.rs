#[derive(Debug)]
pub enum TodoOperation {
    List,
    Add(String),
    Remove,
}
pub struct TodoOptions {
    pub operation: TodoOperation,
}

pub struct Todo {
    pub items: Vec<String>,
    pub options: TodoOptions,
}

impl Todo {
    // Takes an object that implements an Iterator that iterates over Strings
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<TodoOptions, String> {
        args.next(); // skip the program name
        let command = match args.next() {
            Some(command) => command,
            None => return Err("No command provided".to_owned()),
        };


        match command.as_str() {
            "list" => Self::list(args),
            "add" => Self::add(args),
            "remove" => Self::remove(args),
            _ => Err(format!("Invalid command '{}'", command)),
        }
    }

    fn list(args: impl Iterator<Item = String>) -> Result<TodoOptions, String> {
        if args.count() > 0 {
            return Err("List command does not take any parameters".to_owned());
        }

        Ok(TodoOptions {
            operation: TodoOperation::List,
        })
    }

    fn add(mut args: impl Iterator<Item = String>) -> Result<TodoOptions, String> {

        let todo_title = match args.next() {
            Some(parameter) => parameter,
            None => return Err("No parameter provided".to_owned()),
        };


        match args.next() {
            Some(_) => return Err("Add command only takes one parameter".to_owned()),
            None => (),
        }

        Ok(TodoOptions {
            operation: TodoOperation::Add(todo_title),
        })
    }

    fn remove(mut args: impl Iterator<Item = String>) -> Result<TodoOptions, String> {
        
        let to_remove = match args.next() {
            Some(parameter) => parameter,
            None => return Err("No parameter provided".to_owned()),
        };

        Ok(TodoOptions {
            operation: TodoOperation::Remove,
        })
    }
}