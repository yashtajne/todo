
use std::io::{stdout, Write};
use crossterm::{
    queue, execute, cursor::{self, MoveTo, position},
    event::{read, Event, KeyCode, KeyModifiers},
    terminal::{enable_raw_mode, disable_raw_mode},
    style::{
        Color, Stylize, Print,
        // SetBackgroundColor, SetForegroundColor,
        // Attribute, SetAttribute
    }
};

use crate::tui::opt::{Todo, Task, Mode, ListOptions};
use crate::tui::status::Status;

pub struct App {
    todo: Todo,
}


impl App {
    pub fn new(todo: Todo) -> Self { Self { todo } }

    pub fn run(&mut self) {
        enable_raw_mode().expect("");
        let mut stdout = stdout();
        execute!(stdout, cursor::Hide).unwrap();

        let mut wanna_quit = false;
        let mut ignore_binds = false;
        let mut list_options = ListOptions {
            cur: Some(0),
            mode: Mode::Normal,
        };

        loop {
            let res = self.todo.list(&stdout, &list_options);
            if res.is_ok() {
                queue!(
                    stdout,
                    MoveTo(0, position().unwrap().1 - (4 + self
                        .todo
                        .tasks
                        .len()) as u16
                    )
                ).unwrap();
            } else {
                let _ = res.map_err(|e| {
                    execute!(stdout,
                        Print(
                            format!("Error: {}", e)
                                .bold()
                                .on(Color::Red)
                                .with(Color::Rgb {r: 255, g: 255, b: 255})
                        ),
                        MoveTo(0, position().unwrap().1)
                    ).unwrap();
                });
            }

            if wanna_quit { break; }
            let mut quit = || { wanna_quit = true; };

            if let Ok(Event::Key(key)) = read() {
                match key.code {
                    /* ------------ Quit --------------- */
                    KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL
                        => { quit() },
                    KeyCode::Char('q') if !ignore_binds => { quit() },

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

                        let prev_pos = position().unwrap();
                        for _ in 0..=(
                            if self.todo.tasks.is_empty() { 1 }
                            else { self.todo.tasks.len() }
                        ) + 4 {
                            let fill = self.todo.tasks_cell_width + 10;
                            queue!(
                                stdout,
                                Print(
                                    format!("{}\n", " ".repeat(
                                        if fill < 16 { 16 } else { fill }
                                    )
                                )),
                                MoveTo(0, position().unwrap().1)
                            ).unwrap();
                        }
                        queue!(
                            stdout,
                            MoveTo(prev_pos.0 as u16, prev_pos.1 as u16)
                        ).unwrap();
                        stdout.flush().unwrap();

                        list_options.cur = if !self.todo.tasks.is_empty() {
                            Some(self.todo.tasks.len() - 1)
                        } else { None }
                    }

                    /* ------------ Create ------------- */
                    KeyCode::Char('a') if !ignore_binds => {
                        ignore_binds = true;
                        list_options.mode = Mode::Insert;
                        self.todo.tasks.push(Task { status: Status::Create, task: String::new() });
                        list_options.cur = Some(self.todo.tasks.len() - 1);
                        continue;
                    }
                    KeyCode::Enter if ignore_binds => {
                        ignore_binds = false;
                        list_options.mode = Mode::Normal;
                        if let Some(mut t) = self.todo.tasks.pop() {
                            if !t.task.is_empty() {
                                t.status.set(Status::Pending);
                                self.todo.add(&t).unwrap_or_else(|e| { eprintln!("e -> {}", e) });
                                self.todo.tasks.push(t);
                            } else {
                                self.todo.remove(self.todo.tasks.len() - 1).unwrap();
                                list_options.cur = if !self.todo.tasks.is_empty() {
                                    Some(self.todo.tasks.len() - 1)
                                } else { None }
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        if ignore_binds
                            && let Some(l) = self.todo.tasks.last_mut() { l.task.pop(); }
                    }

                    /* ---------- Exit Mode ---------- */
                    KeyCode::Esc => {
                        if ignore_binds {

                            let prev_pos = position().unwrap();
                            for _ in 0..=(
                                if self.todo.tasks.is_empty() { 1 }
                                else { self.todo.tasks.len() }
                            ) + 4 {
                                let fill = self.todo.tasks_cell_width + 10;
                                queue!(
                                    stdout,
                                    Print(
                                        format!("{}\n", " ".repeat(
                                            if fill < 16 { 16 } else { fill }
                                        )
                                    )),
                                    MoveTo(0, position().unwrap().1)
                                ).unwrap();
                            }
                            queue!(
                                stdout,
                                MoveTo(prev_pos.0 as u16, prev_pos.1 as u16)
                            ).unwrap();
                            stdout.flush().unwrap();

                            self.todo.tasks.remove(self.todo.tasks.len() - 1);
                            ignore_binds = false;
                            continue;
                        }
                        break;
                    }
                    _ => {}
                }

                if ignore_binds
                    && let Some(l) = self.todo.tasks.last_mut()
                    && let Some(c) = key.code.as_char() {
                    l.task.push(c);
                }

                self.todo.refresh();
            }
        }

        if !self.todo.tasks.is_empty() {
            queue!(
                stdout,
                MoveTo(0, position().unwrap().1 + (4 + self.todo.tasks.len()) as u16)
            ).unwrap();
        } else { queue!(stdout, MoveTo(0, position().unwrap().1 + 1)).unwrap(); }
        queue!(stdout, cursor::Show).unwrap();
        stdout.flush().unwrap();

        disable_raw_mode().expect("error");
    }
}
