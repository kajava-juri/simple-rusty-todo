mod db;
use db::Database;
use db::models::TodoModel;

#[derive(Debug)]
pub enum TodoOperation {
    List,
    Add(String),
    Remove(i64),
}

pub struct TodoOptions {
    pub operation: TodoOperation,
}

pub struct Todo {
    pub items: Vec<String>,
    pub options: TodoOptions,
    db: Database,
}

impl Todo {
    // Takes an object that implements an Iterator that iterates over Strings
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Todo, String> {
        args.next(); // skip the program name
        let command = match args.next() {
            Some(command) => command,
            None => return Err("No command provided".to_owned()),
        };

        let db = Database::init().map_err(|err| format!("Problem initializing the database: {}", err))?;

        let options = match command.as_str() {
            "list" => Self::parse_list_command(args)?,
            "add" => Self::parse_add_command(args)?,
            "remove" => Self::parse_remove_command(args)?,
            _ => return Err(format!("Invalid command '{}'", command)),
        };

        Ok(Todo {
            items: vec![],
            options,
            db,
        })
    }

    fn parse_list_command(args: impl Iterator<Item = String>) -> Result<TodoOptions, String> {
        if args.count() > 0 {
            return Err("List command does not take any parameters".to_owned());
        }
        Ok(TodoOptions {
            operation: TodoOperation::List,
        })
    }

    fn parse_add_command(mut args: impl Iterator<Item = String>) -> Result<TodoOptions, String> {
        let todo_title = match args.next() {
            Some(parameter) => parameter,
            None => return Err("No parameter provided for add command".to_owned()),
        };

        match args.next() {
            Some(_) => return Err("Add command only takes one parameter".to_owned()),
            None => (),
        }

        Ok(TodoOptions {
            operation: TodoOperation::Add(todo_title),
        })
    }

    fn parse_remove_command(mut args: impl Iterator<Item = String>) -> Result<TodoOptions, String> {
        
        let to_remove = match args.next() {
            Some(parameter) => match parameter.parse::<i64>() {
                Ok(todo_id) => todo_id,
                Err(_) => return Err(format!("Invalid remove command parameter '{}'", parameter)),
            },
            None => return Err("No parameter provided for remove command".to_owned()),
        };

        Ok(TodoOptions {
            operation: TodoOperation::Remove(to_remove),
        })
    }

    pub fn execute(&self) -> Result<(), String> {
        match self.options.operation {
            TodoOperation::List => self.execute_list(),
            TodoOperation::Add(ref parameter) => self.execute_add(parameter),
            TodoOperation::Remove(parameter) => self.execute_remove(parameter),
        }
    }

    fn execute_list(&self) -> Result<(), String> {
        self.db.list_todos().map_err(|err| format!("Problem listing todos: {}", err))
    }

    fn execute_add(&self, parameter: &String) -> Result<(), String> {
        let todo = TodoModel {
            id: 0, // Placeholder, will be set in add_todo
            title: parameter.clone(),
            description: "No description".to_owned(),
            completed: false,
        };
        let inserted_todo = self.db.add_todo(todo).map_err(|err| format!("Problem adding todo: {}", err))?;
        println!("Added todo with ID: {}", inserted_todo.id);
        Ok(())
    }

    fn execute_remove(&self, parameter: i64) -> Result<(), String> {
        self.db.remove_todo(parameter).map_err(|err| format!("Problem removing todo: {}", err))?;
        Ok(())
    }
}