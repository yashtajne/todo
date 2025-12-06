
use std::io::{stdout, Write};
use crossterm::{
    execute, cursor::{self, MoveTo,  position},
    event::{read, Event, KeyCode, KeyModifiers},
    terminal::{enable_raw_mode, disable_raw_mode},
    style::{
        Color, ResetColor,
        SetBackgroundColor, SetForegroundColor,
        Attribute, SetAttribute
    }
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

        let mut wanna_quit = false;
        let mut ignore_binds = false;
        let mut list_options = ListOptions {
            cur: Some(self.cur),
            draw_color: if ignore_binds { Color::Red } else { Color::Black }
        };

        // let prev_buff_pos = position().unwrap();
        // let prev_buff_size = (0, 0);
        loop {
            if ignore_binds {
                list_options.draw_color = Color::Green;
            } else { list_options.draw_color = Color::Black; }

            let res = self.todo.list(&stdout, &list_options);
            if res.is_ok() {
                execute!(stdout, MoveTo(0, position().unwrap().1 - (4 + self.todo.tasks.len()) as u16)).unwrap();
            } else {
                let _ = res.map_err(|e| {
                    execute!(stdout,
                        SetAttribute(Attribute::Bold),
                        SetBackgroundColor(Color::Red),
                        SetForegroundColor(Color::Rgb {r: 255, g: 255, b: 255}),
                    ).unwrap();
                    write!(stdout, "Error: {}", e).unwrap();
                    execute!(stdout,
                        ResetColor,
                        SetAttribute(Attribute::Reset),
                        MoveTo(0, position().unwrap().1)
                    ).unwrap();
                });
            }

            if wanna_quit { break; }

            if let Ok(Event::Key(key)) = read() {
                match key.code {
                    KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL
                        && !ignore_binds => { wanna_quit = true },
                    KeyCode::Char('q') if !ignore_binds => { wanna_quit = true },
                    KeyCode::Char('j') if !ignore_binds => {
                        if self.cur != 0 || self.cur != self.todo.tasks.len() - 1 { self.cur += 1; }
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
                        continue;
                    }
                    KeyCode::Enter if ignore_binds => {
                        ignore_binds = false;
                        self.todo.add(self.todo.tasks.last().unwrap().to_owned())
                            .unwrap_or_else(|e| { eprintln!("e -> {}", e) });
                    }
                    KeyCode::Esc => {
                        if ignore_binds {
                            let prev_pos = position().unwrap();
                            execute!(
                                stdout,
                                MoveTo(0, position().unwrap().1 + (4 + self.todo.tasks.len()) as u16),
                            ).unwrap();
                            writeln!(stdout, "{}", " ".repeat(self.todo.tasks_cell_width + 10)).unwrap();
                            execute!(stdout, MoveTo(prev_pos.0 as u16, prev_pos.1 as u16)).unwrap();
                            self.todo.tasks.remove(self.todo.tasks.len() - 1);
                            ignore_binds = false;
                            wanna_quit = true;
                            continue;
                        }

                        break;
                    }
                    KeyCode::Backspace => {
                        if ignore_binds
                            && let Some(l) = self.todo.tasks.last_mut() { l.pop(); }
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

        if !self.todo.tasks.is_empty() {
            execute!(stdout, MoveTo(0, position().unwrap().1 + (4 + self.todo.tasks.len()) as u16)).unwrap();
        } else { execute!(stdout, MoveTo(0, position().unwrap().1 + 1)).unwrap(); }
        execute!(stdout, cursor::Show).unwrap();
        disable_raw_mode().expect("");
    }
}
