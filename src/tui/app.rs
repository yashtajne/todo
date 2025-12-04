
use std::io::{stdout, Write};
use crossterm::{
    execute, cursor::{self, MoveTo,  position},
    event::{read, Event, KeyCode, KeyModifiers},
    terminal::{enable_raw_mode, disable_raw_mode}
};

use crate::todo::{Todo, ListOptions};

pub struct App {
    todo: Todo,
    cur:  usize,
}


impl App {
    pub fn new(todo: Todo) -> Self { Self { cur: 0, todo } }

    pub fn run(&self) {
        enable_raw_mode().expect("");
        let mut stdout = stdout();
        execute!(stdout, cursor::Hide).unwrap();

        loop {
            self.todo.list(&stdout, ListOptions {
                cur: Some(self.cur)
            }).unwrap_or_else(|e| { eprintln!("e -> {}", e) });

            stdout.flush().unwrap();

            if let Ok(Event::Key(key)) = read() {
                match key.code {
                    KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => {
                        break;
                        },
                    KeyCode::Char('j') => {}
                    KeyCode::Char('k') => {}
                    _ => {}
                }

                self.todo.list(&stdout, ListOptions {
                    cur: Some(self.cur)
                }).unwrap_or_else(|e| { eprintln!("e -> {}", e) });

                stdout.flush().unwrap();
            }
        }

        execute!(stdout, MoveTo(0, position().unwrap().1 + (4 + self.todo.tasks.len()) as u16)).unwrap();
        execute!(stdout, cursor::Show).unwrap();
        disable_raw_mode().expect("");
    }
}
