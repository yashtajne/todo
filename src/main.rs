
use std::env;
use std::io::stdout;
use std::fs::OpenOptions;

mod todo;
use todo::{Todo, Mode, ListOptions};

mod tui;
use tui::app::{App};

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
            let stdout = stdout();
            todo.list(stdout, &ListOptions {
                cur: None,
                mode: Mode::Normal
            })
                .unwrap_or_else(|e| {
                    panic!("\n[Error occured] -> {}", e);
                });
        }
        _ => {
            let mut app = App::new(todo);
            app.run();
        }
    }
}
