
use std::env;
use std::fs::OpenOptions;

mod todo;
use todo::*;

macro_rules! help_msg {
    () => {
        println!(r#"
 use case:
     todo [operation] [argument]

 operations:
     --list   Lists all Tasks in to-do (no argument)
     --add    Adds task to to-do (requires 1 argument)
     --remove Removes a task with the provided task ID (requires 1 argument)
     --help   Prints help message

 examples:
     todo --list
         Lists all the tasks

     todo --add "Touch grass"
         Adds the task

     todo --remove 1
         Removes task which has ID 1
        "#);
    }
}

static TODO_FILE: &str = concat!(env!("HOME"), "/.todo");

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(TODO_FILE)
        .expect("Failed to Open ~/.todo file!");

    let mut todo = Todo::init(file).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    match args.get(1).map(|s| s.as_str()) {
        Some("--help") => {
            help_msg!();
        }
        Some("--add") => {
            if args.len() <= 2 {
                panic!("Empty task!?");
            }
            todo.add(args[2].clone())
                .unwrap_or_else(|e| {
                    panic!("\n[Error occured] -> {}", e);
                });
        }
        Some("--remove") => {
            if args.len() <= 2 {
                panic!("Task ID not provided!");
            }
            let _ = todo.remove(args[2]
                .clone()
                .trim()
                .parse()
                .unwrap_or_else(|e| {
                    panic!("\n[Error occured] -> {}", e);
                }));
        }
        Some("--list") => {
            todo.list()
                .unwrap_or_else(|e| {
                    panic!("\n[Error occured] -> {}", e);
                });
        }
        _ => { help_msg!(); }
    }
}
