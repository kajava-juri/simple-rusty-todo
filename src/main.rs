use std::{clone, env, process};
use simple_rusty_todo::Todo;
use simple_rusty_todo::TodoOptions;

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

    // print the options from the todo object
    println!("{:?}", todo.operation);
    println!("{:?}", todo.parameter);
    
}
