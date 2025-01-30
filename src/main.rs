use std::{env, process};
use simple_rusty_todo::Todo;

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

    todo.execute().unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
}
