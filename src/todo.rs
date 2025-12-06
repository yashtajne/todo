
use std::fs::File;
use std::io::{Read, Write, SeekFrom, Seek};
use crossterm::{
    execute, cursor::{MoveTo, position},
    style::{SetForegroundColor, Color,  ResetColor}
};

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

pub struct Todo {
    pub tasks: Vec<String>,
    pub file:  File,
    pub tasks_cell_width: usize,
}

pub struct ListOptions {
    pub draw_color: Color,
    pub cur: Option<usize>,
}

impl Todo {
    pub fn refresh(&mut self) {
         self.tasks_cell_width = self.tasks
             .iter()
             .map(|s| s.len())
             .max()
             .unwrap_or(0);
    }
    pub fn init(mut file: File) -> Result<Self, String> {
        let mut tasks: Vec<String> = Vec::new();
        let mut buff: Vec<u8> = Vec::new();

        if file.read_to_end(&mut buff).is_err() {
            return Err("Error while Reading ~/.todo file!".to_string());
        }

        if !buff.is_empty() {
            let mut cur = 0;
            for i in 0..=buff.len() - 1 {
                if buff[i] == b'\n' {
                    tasks.push(
                        String::from_utf8(buff[cur..i].to_vec())
                            .expect("Error while reading tasks!")
                    );
                    cur = i + 1;
                }
            }
        }

        let tasks_cell_width = tasks
            .iter()
            .map(|s| s.len())
            .max()
            .unwrap_or(0);

        Ok(Self { tasks, file, tasks_cell_width })
    }

    pub fn add(&mut self, task: String) -> Result<(), String> {
        self.file.write_all((task.trim().to_owned() + "\n").as_bytes())
            .unwrap_or_else(|_e| { "Error while adding task!".to_string(); });

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
            self.file.write_all((task.trim().to_owned() + "\n").as_bytes())
                .unwrap_or_else(|_e| { "Error while adding task!".to_string(); });
        }

        Ok(())
    }

    pub fn list<W: Write>(&self, mut w: W, options: &ListOptions) -> Result<(), String> {
        if self.tasks_cell_width == 0
            || self.tasks.is_empty() { return Err("No Tasks!".to_string()); }

        execute!(w,
            SetForegroundColor(options.draw_color),
        ).unwrap();

        writeln!(w, "┏━━━━━┳━{}━┓ ", "━".repeat(self.tasks_cell_width)).unwrap();
        execute!(w, MoveTo(0, position().unwrap().1)).unwrap();

        writeln!(w, "┃ IDs ┃ Tasks{} ┃ ", " ".repeat(
            if self.tasks_cell_width.saturating_sub(5) == 0 { 0 } else { self.tasks_cell_width - 5 }
        )).unwrap();
        execute!(w, MoveTo(0, position().unwrap().1)).unwrap();

        writeln!(w, "┣━━━━━╋━{}━┫ ", "━".repeat(self.tasks_cell_width)).unwrap();
        execute!(w, MoveTo(0, position().unwrap().1)).unwrap();

        for i in 0..=self.tasks.len() - 1 {
            let task = &self.tasks[i];
            let task_cell_length = task.len();

            if let Some(j) = options.cur && i == j {}
//               execute!(w,
//                   SetBackgroundColor(Color::White),
//                   SetForegroundColor(options.draw_color),
//               ).unwrap();
//
//               writeln!(w, "┃ {:2}  ┃ {} ┃",
//                   i, task.to_owned() + &" ".repeat(self.tasks_cell_width - task_cell_length)
//               ).unwrap();
//
//               execute!(w,
//                   MoveTo(0, position().unwrap().1),
//                   ResetColor
//               ).unwrap();
//
//               continue;
//           }


            writeln!(w, "┃ {:2}  ┃ {} ┃ ",
                i, task.to_owned() + &" ".repeat(self.tasks_cell_width - task_cell_length)
            ).unwrap();

            execute!(w, MoveTo(0, position().unwrap().1)).unwrap();
        }
        execute!(w, MoveTo(0, position().unwrap().1)).unwrap();

        writeln!(w, "┗━━━━━┻━{}━┛ ", "━".repeat(self.tasks_cell_width)).unwrap();
        execute!(w, ResetColor).unwrap();

        Ok(())
    }
}
