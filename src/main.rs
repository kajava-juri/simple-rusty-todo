use std::{env, process};
use simple_rusty_todo::Todo;
mod db;
use db::models::Todo_model;
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
        TodoOperation::Add => {
            let todo = Todo_model {
                title: todo.parameter,
                description: "No description".to_owned(),
                completed: false,
            };
            db.add_todo(todo).unwrap_or_else(|err| {
                eprintln!("Problem adding todo: {}", err);
                process::exit(1);
            });
        }
        TodoOperation::Remove => {
            println!("Remove");
        }
    }
    
}
