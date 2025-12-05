
use std::io::{stdout, Write};
use crossterm::{
    execute, style::{Color},
    cursor::{self, MoveTo,  position},
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

    pub fn run(&mut self) {
        enable_raw_mode().expect("");
        let mut stdout = stdout();
        execute!(stdout, cursor::Hide).unwrap();

        let mut ignore_binds = false;
        let mut list_options = ListOptions {
            cur: Some(self.cur),
            draw_color: if ignore_binds { Color::Red } else { Color::Black }
        };

        loop {
            self.todo.list(&stdout, &list_options).unwrap_or_else(|e| { eprintln!("e -> {}", e) });

            if let Ok(Event::Key(key)) = read() {
                match key.code {
                    KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL && !ignore_binds => { break; },
                    KeyCode::Char('q') if !ignore_binds => { break; },
                    KeyCode::Char('j') if !ignore_binds => {
                        if self.cur != self.todo.tasks.len() - 1 { self.cur += 1; }
                    }
                    KeyCode::Char('k') if !ignore_binds => { if self.cur != 0 { self.cur -= 1; } }
                    KeyCode::Char('d') if !ignore_binds => {
                        self.todo.remove(self.cur).unwrap();
                        let prev_pos = position().unwrap();
                        execute!(
                            stdout,
                            MoveTo(0, position().unwrap().1 + (4 + self.todo.tasks.len()) as u16),
                        ).unwrap();
                        writeln!(stdout, "{}", " ".repeat(self.todo.tasks_cell_width + 10)).unwrap();
                        execute!(stdout, MoveTo(prev_pos.0 as u16, prev_pos.1 as u16)).unwrap();
                    }
                    KeyCode::Char('a') if !ignore_binds => {
                        ignore_binds = true;
                        list_options.draw_color = Color::Green;
                        self.todo.tasks.push(String::new());
                    }
                    KeyCode::Enter => {
                        ignore_binds = false;
                        list_options.draw_color = Color::Black;
                    }
                    _ => {}
                }

                if ignore_binds
                    && let Some(l) = self.todo.tasks.last_mut()
                    && let Some(c) = key.code.as_char() {
                        l.push(c);
                }
                self.todo.refresh();
            }
        }

        execute!(stdout, MoveTo(0, position().unwrap().1 + (4 + self.todo.tasks.len()) as u16)).unwrap();
        execute!(stdout, cursor::Show).unwrap();
        disable_raw_mode().expect("");
    }
}
