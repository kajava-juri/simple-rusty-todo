
#[derive(Debug)]
pub enum TodoOperation {
    List,
    Add,
    Remove,
}
pub struct TodoOptions {
    pub operation: TodoOperation,
    pub parameter: String,
}

pub struct Todo {
    pub items: Vec<String>,
    pub options: TodoOptions
}

impl Todo {
    // Takes an object that implements an Iterator that iterates over Strings
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<TodoOptions, String> {
        args.next(); // skip the program name
        let command = match args.next() {
            Some(command) => Todo::match_command(command)?,
            None => return Err("No command provided".to_owned()),
        };

        // TODO: Change parameter parsing, list does not need a parameter

        let parameter = match args.next() {
            Some(parameter) => parameter,
            None => return Err("No parameter provided".to_owned()),
        };

        Ok(TodoOptions {
            operation: command,
            parameter,
        })
    }

    

    fn match_command(command: String) -> Result<TodoOperation, String> {
        match command.as_str() {
            "list" => Ok(TodoOperation::List),
            "add" => Ok(TodoOperation::Add),
            "remove" => Ok(TodoOperation::Remove),
            _ => Err(format!("Invalid command '{}'", command)),
        }
    }
}