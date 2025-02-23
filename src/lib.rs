mod db;
mod tests;
use db::Database;
use db::models::TodoModel;

#[derive(Debug)]
struct ListOperation;

struct AddOperation {
    title: String,
}

struct RemoveOperation {
    id: i64,
}

#[allow(dead_code)]
struct UpdateOperation {
    id: i64,
    new_item: String,
}

#[allow(dead_code)]
enum TodoOperation {
    List(ListOperation),
    Add(AddOperation),
    Remove(RemoveOperation),
    Update(UpdateOperation),
}

pub struct TodoOptions {
    operation: TodoOperation,
}

pub struct Todo {
    pub items: Vec<String>,
    pub options: TodoOptions,
    db: Database,
}

trait Command {
    fn parse(args: impl Iterator<Item = String>) -> Result<TodoOperation, String>;
}

impl Command for ListOperation {
    fn parse(args: impl Iterator<Item = String>) -> Result<TodoOperation, String> {
        if args.count() > 0 {
            return Err("List command does not take any parameters".to_owned());
        }
        Ok(TodoOperation::List(ListOperation))
    }
}

impl Command for AddOperation {
    fn parse(mut args: impl Iterator<Item = String>) -> Result<TodoOperation, String> {
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
}

impl Command for RemoveOperation {
    fn parse(mut args: impl Iterator<Item = String>) -> Result<TodoOperation, String> {
        
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
            "list" => ListOperation::parse(args)?,
            "add" => AddOperation::parse(args)?,
            "remove" => RemoveOperation::parse(args)?,
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