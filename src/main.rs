
use std::env;
use std::io::stdout;
use std::fs::OpenOptions;

mod todo;
mod tui;

use tui::app::{App};
use tui::opt::{Todo, Task, Mode, ListOptions};
use tui::status::Status;

static TODO_FILE: &str = concat!(env!("HOME"), "/.todo");

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
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
            todo.add(&Task { task: args[2].clone(), status: Status::Pending })
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
            let stdout = stdout();
            todo.list(stdout, &ListOptions {
                cur: None,
                mode: Mode::Normal
            })
                .unwrap_or_else(|e| {
                    panic!("\n[Error occured] -> {}", e);
                });
        }
        _ if args.len() == 1 => {
            let mut app = App::new(todo);
            app.run();
        }
        _ => {
            panic!("\n[Error occured] -> Invalid args!");
        }
    }
}
