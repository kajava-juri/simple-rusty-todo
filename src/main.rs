use std::{env, process};
use simple_rusty_todo::Todo;
mod db;
use db::models::TodoModel;
use simple_rusty_todo::TodoOperation;

fn main() {
    let args = env::args();
    for arg in env::args() {
        println!("{}", arg);
    }
    println!("-----------------");
    let todo = Todo::build(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let db = db::Database::init().unwrap_or_else(|err| {
        eprintln!("Problem initializing the database: {}", err);
        process::exit(1);
    });

    match todo.operation {
        TodoOperation::List => {
            db.list_todos().unwrap_or_else(|err| {
                eprintln!("Problem listing todos: {}", err);
                process::exit(1);
            });
        }
        TodoOperation::Add(parameter) => {
            let todo = TodoModel {
                id: 0, // Placeholder, will be set in add_todo
                title: parameter,
                description: "No description".to_owned(),
                completed: false,
            };
            let inserted_todo = db.add_todo(todo).unwrap_or_else(|err| {
                eprintln!("Problem adding todo: {}", err);
                process::exit(1);
            });
            println!("Added todo with ID: {}", inserted_todo.id);
        }
        TodoOperation::Remove => {
            println!("Remove");
        }
    }
    
}
