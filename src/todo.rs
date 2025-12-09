
use std::fs::File;
use std::io::{Read, Write, SeekFrom, Seek};
use crossterm::{
    queue,
    cursor::{MoveTo, position},
    style::{
        Print, Stylize,
        Color,  ResetColor,
        // SetBackgroundColor,
        // SetForegroundColor
    }
};

use crate::tui::opt::{Todo, Task, Mode, ListOptions};
use crate::tui::status::Status;

#[macro_export]
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

impl Todo {
    pub fn refresh(&mut self) {
        let w = self.tasks
             .iter()
             .map(|s| s.task.len())
             .max()
             .unwrap_or(0);
        self.tasks_cell_width = if w <= 5 { 5 } else { w };
    }

    pub fn init(mut file: File) -> Result<Self, String> {
        let mut tasks: Vec<Task> = Vec::new();
        let mut buff: Vec<u8> = Vec::new();

        if file.read_to_end(&mut buff).is_err() {
            return Err("Error while Reading ~/.todo file!".to_string());
        }

        if !buff.is_empty() {
            let mut cur = 0;
            for i in 0..=buff.len() - 1 {
                if buff[i] == b'\n' {
                    let line = String::from_utf8(buff[cur..i].to_vec())
                            .expect("Error while reading tasks!");
                    let (s, t) = line.split_at(1);
                    let status = s.parse::<u8>().unwrap_or_else(|e| { panic!("Error while getting status: String({}) Err({})", s, e) });
                    tasks.push(
                        Task{
                            task: t.to_string(),
                            status: Status::get_enum(status)
                        }
                    );
                    cur = i + 1;
                }
            }
        }

        let tasks_cell_width = tasks
            .iter()
            .map(|s| s.task.len())
            .max()
            .unwrap_or(0);

        Ok(Self { tasks, file, tasks_cell_width })
    }

    pub fn add(&mut self, task: &Task) -> Result<(), String> {
        self.file.write_all(
            format!(
                "{}{}\n",
                task.status.get_code(),
                task.task.trim()
            ).as_bytes()
        ).unwrap();

        Ok(())
    }

    pub fn remove(&mut self, taskid: usize) -> Result<(), String> {
        if taskid > self.tasks.len() {
            return Err("No task with the prvided ID".to_string())
        }

        self.tasks.remove(taskid);

        if self.file.set_len(0).is_err() {
            return Err("Error while truncating todo file!".to_string())
        }

        self.file.seek(SeekFrom::Start(0))
            .map_err(|_| "seek failed".to_string())?;

        for task in &self.tasks {
            self.file.write_all((task.task.trim().to_owned() + "\n").as_bytes())
                .unwrap_or_else(|_e| { "Error while adding task!".to_string(); });
        }

        Ok(())
    }

    pub fn list<W: Write>(&self, mut w: W, options: &ListOptions) -> Result<(), String> {
        if self.tasks_cell_width == 0
            || self.tasks.is_empty() { return Err("No Tasks!".to_string()); }

        queue!(
            w,
            Print(format!("┏━━━━━┳━{}━┳━━━━━━━━━━━┓ \n", "━".repeat(
                if self.tasks_cell_width < 5 { 5 }
                else { self.tasks_cell_width }
            ))),
            MoveTo(0, position().unwrap().1),
            Print(format!("┃ IDs ┃ Tasks{} ┃ Status    ┃ \n", " ".repeat(
                if self.tasks_cell_width.saturating_sub(5) == 0 { 0 }
                else { self.tasks_cell_width - 5 }
            ))),
            MoveTo(0, position().unwrap().1),
            Print(format!("┣━━━━━╋━{}━╋━━━━━━━━━━━┫ \n", "━".repeat(
                if self.tasks_cell_width < 5 { 5 }
                else { self.tasks_cell_width }
            ))),
            MoveTo(0, position().unwrap().1),
        ).unwrap();

        for i in 0..=self.tasks.len() - 1 {
            let task = &self.tasks[i].task;
            let status = &self.tasks[i].status;
            let is_in_insert_mode = matches!(options.mode, Mode::Insert);

            let s = status.get_string();
            let r = 9 - (s.len() as i8);

            // println!("{}{}", s, r);

            queue!(w,
                Print("┃"),
                Print(
                    format!(" {:2}  ", i)
                        .bold()
                        .on(
                            if options.cur == Some(i) {
                                if is_in_insert_mode { Color::Green }
                                else { Color::DarkGrey }
                            }
                            else { Color::Reset }
                        )
                        .with(
                            if is_in_insert_mode { Color::Rgb { r: 255, g: 255, b: 255} }
                            else { Color::Reset }
                        )
                ),
                Print("┃"),
                Print(
                    format!(" {}{} ", task.to_owned(), " ".repeat(
                            self.tasks_cell_width.saturating_sub(task.len())
                        )
                    )
                    .bold()
                    .on(
                        if options.cur == Some(i) {
                            if is_in_insert_mode { Color::Green }
                            else { Color::DarkGrey }
                        }
                        else { Color::Reset }
                    )
                    .with(
                        if is_in_insert_mode { Color::Rgb { r: 255, g: 255, b: 255} }
                        else { Color::Reset }
                    )
                ),
                Print("┃"),
                Print(
                    format!(" {}{} ", s, " ".repeat(r as usize))
                    .bold()
                    .on(
                        if options.cur == Some(i) {
                            if is_in_insert_mode { Color::Green }
                            else { Color::DarkGrey }
                        }
                        else { Color::Reset }
                    )
                    .with(
                        if is_in_insert_mode { Color::Rgb { r: 255, g: 255, b: 255} }
                        else { Color::Reset }
                    )
                ),
                Print("┃ \n"),
                MoveTo(0, position().unwrap().1),
                ResetColor,
            ).unwrap();
        }

        queue!(
            w,
            Print(format!("┗━━━━━┻━{}━┻━━━━━━━━━━━┛ \n", "━".repeat(
                if self.tasks_cell_width < 5 { 5 }
                else { self.tasks_cell_width }
            ))),
            ResetColor
        ).unwrap();

        w.flush().unwrap();

        Ok(())
    }
}
