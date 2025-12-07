
use std::io::{stdout, Write};
use crossterm::{
    queue, execute, cursor::{MoveTo,  position},
    event::{read, Event, KeyCode, KeyModifiers},
    terminal::{enable_raw_mode, disable_raw_mode},
    style::{
        Color, ResetColor, Print,
        SetBackgroundColor, SetForegroundColor,
        Attribute, SetAttribute
    }
};

use crate::todo::{Todo, Mode, ListOptions};

pub struct App {
    todo: Todo,
}


impl App {
    pub fn new(todo: Todo) -> Self { Self { todo } }

    pub fn run(&mut self) {
        enable_raw_mode().expect("");
        let mut stdout = stdout();
        // execute!(stdout, cursor::Hide).unwrap();

        let mut wanna_quit = false;
        let mut ignore_binds = false;
        let mut list_options = ListOptions {
            cur: Some(0),
            mode: Mode::Normal,
            // bg_col: if ignore_binds { Color::Red }
            //     else { Color::Rgb { r: 0, g: 0, b: 0 } },
            // fg_col: Color::Rgb { r: 255, g: 255, b: 255 }
        };

        loop {

            // if ignore_binds {
            //     list_options.fg_col = Color::Green;
            // } else { list_options.fg_col = Color::Black; }

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
                    /* ------------ Quit --------------- */
                    KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL
                        => { wanna_quit = true },
                    KeyCode::Char('q') if !ignore_binds => { wanna_quit = true },

                    /* ---------- Navigation ----------- */
                    KeyCode::Char('j') if !ignore_binds => {
                        if let Some(ref mut cur) = list_options.cur
                            && *cur != self.todo.tasks.len() - 1  { *cur += 1; }
                    }
                    KeyCode::Char('k') if !ignore_binds => {
                        if let Some(ref mut cur) = list_options.cur
                            && *cur > 0 { *cur -= 1; }
                    }

                    /* ------------ Delete ------------- */
                    KeyCode::Char('d') if !ignore_binds
                        && !self.todo.tasks.is_empty() => {

                        self.todo.remove(list_options.cur.unwrap()).unwrap();
                        // if self.todo.tasks.is_empty() {
                        //     list_options.cur = None;
                        //     continue;
                        // };

                        let prev_pos = position().unwrap();
                        for _ in 0..=(
                            if self.todo.tasks.is_empty() { 1 }
                            else { self.todo.tasks.len() }
                        ) + 4 {
                            queue!(
                                stdout,
                                Print(format!("{}\n", " ".repeat(self.todo.tasks_cell_width + 10))),
                                MoveTo(0, position().unwrap().1)
                            ).unwrap();
                        }
                        queue!(stdout, MoveTo(prev_pos.0 as u16, prev_pos.1 as u16)).unwrap();
                        stdout.flush().unwrap();

                        list_options.cur = if !self.todo.tasks.is_empty() {
                            Some(self.todo.tasks.len() - 1)
                        } else { None }
                    }

                    /* ------------ Create ------------- */
                    KeyCode::Char('a') if !ignore_binds => {
                        ignore_binds = true;
                        list_options.mode = Mode::Insert;
                        self.todo.tasks.push(String::new());
                        list_options.cur = Some(self.todo.tasks.len() - 1);
                        continue;
                    }
                    KeyCode::Enter if ignore_binds => {
                        ignore_binds = false;
                        list_options.mode = Mode::Normal;
                        if !self.todo.tasks.last().unwrap().is_empty() {
                            self.todo.add(self.todo.tasks.last().unwrap().to_owned())
                                .unwrap_or_else(|e| { eprintln!("e -> {}", e) });
                        } else {
                            self.todo.remove(self.todo.tasks.len() - 1).unwrap();
                            list_options.cur = if !self.todo.tasks.is_empty() {
                                Some(self.todo.tasks.len() - 1)
                            } else { None }
                        }
                    }
                    KeyCode::Backspace => {
                        if ignore_binds
                            && let Some(l) = self.todo.tasks.last_mut() { l.pop(); }
                    }

                    /* ---------- Exit Mode ---------- */
                    KeyCode::Esc => {
                        if ignore_binds {
                            let prev_pos = position().unwrap();
                            execute!(
                                stdout,
                                MoveTo(0, position().unwrap().1 + (3 + self.todo.tasks.len()) as u16),
                                Print(" ".repeat(self.todo.tasks_cell_width + 10)),
                                MoveTo(prev_pos.0 as u16, prev_pos.1 as u16)
                            ).unwrap();
                            self.todo.tasks.remove(self.todo.tasks.len() - 1);
                            ignore_binds = false;
                            wanna_quit = true;
                            continue;
                        }
                        break;
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
        // execute!(stdout, cursor::Show).unwrap();
        disable_raw_mode().expect("");
    }
}
