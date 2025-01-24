
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
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<TodoOptions, &'static str> {
        args.next(); // skip the program name
        let command = match args.next() {
            Some(command) => Todo::match_command(command)?,
            None => return Err("No command provided"),
        };

        let parameter = match args.next() {
            Some(parameter) => parameter,
            None => return Err("No parameter provided"),
        };

        Ok(TodoOptions {
            operation: command,
            parameter,
        })
    }

    fn match_command(command: String) -> Result<TodoOperation, &'static str> {
        match command.as_str() {
            "list" => Ok(TodoOperation::List),
            "add" => Ok(TodoOperation::Add),
            "remove" => Ok(TodoOperation::Remove),
            _ => Err("Invalid command"),
        }
    }
}