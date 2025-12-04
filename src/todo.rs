
use std::fs::File;
use std::io::{Read, Write, SeekFrom, Seek};


pub struct Todo {
    pub tasks: Vec<String>,
    pub file:  File,
}

impl Todo {
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

        Ok(Self { tasks, file })
    }

    pub fn add(&mut self, task: String) -> Result<(), String> {
        if self.tasks.contains(&task) {
            return Err("Task already exists!".to_string())
        }

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

    pub fn list(&self) -> Result<(), String> {
        let tasks_cell_width = self.tasks
            .iter()
            .map(|s| s.len())
            .max()
            .unwrap_or(0);

        if tasks_cell_width == 0 {
            return Err("No Tasks!".to_string());
        }

        println!("┏━━━━━┳━{}━┓", "━".repeat(tasks_cell_width));
        println!("┃ IDs ┃ Tasks{} ┃", " ".repeat(
            if tasks_cell_width - 5 == 0 { 0 } else { tasks_cell_width - 5 }
        ));
        println!("┣━━━━━╋━{}━┫", "━".repeat(tasks_cell_width));

        for i in 0..=self.tasks.len() - 1 {
            let task = &self.tasks[i];
            let task_cell_length = task.len();
            println!("┃ {:2}  ┃ {} ┃",
                i, task.to_owned() + &" ".repeat(tasks_cell_width - task_cell_length)
            );
        }

        println!("┗━━━━━┻━{}━┛", "━".repeat(tasks_cell_width));

        Ok(())
    }
}
