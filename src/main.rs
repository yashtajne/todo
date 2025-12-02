
use std::env;
use std::fs::OpenOptions;

mod todo;
use todo::*;


const TODO_FILE: &str = concat!(env!("HOME"), "/.todo");


fn main() {
    let args: Vec<String> = env::args().collect();

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(TODO_FILE)
        .expect("Failed to Open ~/.todo file!");

    let mut todo = Todo::init(file).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    match args.get(1).map(|s| s.as_str()) {
        Some("--help") => {
            println!(r#"
 use case:
     todo [operation] [argument]

 operations:
     --list   Lists all Tasks in to-do (default operation, no args)
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
        Some("--add") => {
            if args.len() <= 2 {
                panic!("Empty task!?");
            }
            let _ = todo.add(String::from(args[2].clone())).map_err(|e| {
                panic!("{}", e);
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
                .unwrap_or_else(|_| {
                    panic!("Error while parsing task ID!");
                }));
        }
        _ => {
            let tasks_cell_width = todo.tasks
                .iter()
                .map(|s| s.len())
                .max()
                .unwrap_or(0);

            if tasks_cell_width == 0 {
                panic!("No Tasks!");
            }

            println!("┏━{}━┳━{}━┓", "━━━", "━".repeat(tasks_cell_width));
            println!("┃ {} ┃ {} ┃", "IDs", "Tasks".to_string() + &" ".repeat(
                if tasks_cell_width - 5 <= 0 { 0 } else { tasks_cell_width - 5 }
            ));
            println!("┣━{}━╋━{}━┫", "━━━", "━".repeat(tasks_cell_width));

            for i in 0..=todo.tasks.len() - 1 {
                let task = &todo.tasks[i];
                let task_cell_length = task.len();
                println!("┃ {:2}  ┃ {} ┃", i, task.to_owned() + &" ".repeat(tasks_cell_width - task_cell_length));
            }

            println!("┗━{}━┻━{}━┛", "━━━", "━".repeat(tasks_cell_width));
        }
    }
}
