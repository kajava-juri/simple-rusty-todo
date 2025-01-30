mod db;
use std::ops::Add;

use db::Database;
use db::models::TodoModel;
use mongodb::options;

#[derive(Debug)]
struct ListOperation;

struct AddOperation {
    title: String,
}

struct RemoveOperation {
    id: i64,
}

struct UpdateOperation {
    id: i64,
    new_item: String,
}

enum TodoOperation {
    List(ListOperation),
    Add(AddOperation),
    Remove(RemoveOperation),
    Update(UpdateOperation),
}

struct TodoOptions {
    operation: TodoOperation,
}

pub struct Todo {
    pub items: Vec<String>,
    pub options: TodoOptions,
    db: Database,
}

impl Todo {
    // Takes an object that implements an Iterator that iterates over Strings
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Todo, String> {
        // Parse the command line arguments for program options
        args.next(); // skip the program name
        let command = match args.next() {
            Some(command) => command,
            None => return Err("No command provided".to_owned()),
        };

        let db = Database::init().map_err(|err| format!("Problem initializing the database: {}", err))?;

        // Parse the command line arguments for the command
        let operation = match command.as_str() {
            "list" => Self::parse_list_command(args)?,
            "add" => Self::parse_add_command(args)?,
            "remove" => Self::parse_remove_command(args)?,
            _ => return Err(format!("Invalid command '{}'", command)),
        };

        let options = TodoOptions { 
            operation 
        };

        Ok(Todo {
            items: vec![],
            options,
            db,
        })
    }

    // TODO: maybe get rid of command specific parsing functions?

    fn parse_list_command(args: impl Iterator<Item = String>) -> Result<TodoOperation, String> {
        if args.count() > 0 {
            return Err("List command does not take any parameters".to_owned());
        }
        Ok(TodoOperation::List(ListOperation))
    }

    fn parse_add_command(mut args: impl Iterator<Item = String>) -> Result<TodoOperation, String> {
        let todo_title = match args.next() {
            Some(parameter) => parameter,
            None => return Err("No parameter provided for add command".to_owned()),
        };

        match args.next() {
            Some(_) => return Err("Add command only takes one parameter".to_owned()),
            None => (),
        }

        Ok(TodoOperation::Add(AddOperation { 
            title: todo_title 
        }))
    }

    fn parse_remove_command(mut args: impl Iterator<Item = String>) -> Result<TodoOperation, String> {
        
        let to_remove = match args.next() {
            Some(parameter) => match parameter.parse::<i64>() {
                Ok(todo_id) => todo_id,
                Err(_) => return Err(format!("Invalid remove command parameter '{}'", parameter)),
            },
            None => return Err("No parameter provided for remove command".to_owned()),
        };

        Ok(TodoOperation::Remove(RemoveOperation { 
            id: to_remove 
        }))
    }

    pub fn execute(&self) -> Result<(), String> {
        match &self.options.operation {
            TodoOperation::List(ListOperation) => self.list(),
            TodoOperation::Add(options  ) => self.add(options),
            TodoOperation::Remove(options) => self.remove(options),
            TodoOperation::Update(_) => unimplemented!(),
        }
    }

    fn list(&self) -> Result<(), String> {
        self.db.list_todos().map_err(|err| format!("Problem listing todos: {}", err))
    }

    fn add(&self, add: &AddOperation) -> Result<(), String> {
        let todo = TodoModel {
            id: 0, // Placeholder, will be set in add_todo
            title: add.title.clone(),
            description: "No description".to_owned(),
            completed: false,
        };
        let inserted_todo = self.db.add_todo(todo).map_err(|err| format!("Problem adding todo: {}", err))?;
        println!("Added todo with ID: {}", inserted_todo.id);
        Ok(())
    }

    fn remove(&self, remove: &RemoveOperation) -> Result<(), String> {
        self.db.remove_todo(remove.id).map_err(|err| format!("Problem removing todo: {}", err))?;
        Ok(())
    }
}