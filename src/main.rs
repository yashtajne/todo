
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, SeekFrom, Seek};


const TODO_FILE: &str = concat!(env!("HOME"), "/.todo");


struct Todo {
    tasks: Vec<String>,
    file:  File,
}

impl Todo {
    fn init(mut file: File) -> Result<Self, String> {
        let mut tasks: Vec<String> = Vec::new();
        let mut buff: Vec<u8> = Vec::new();

        if file.read_to_end(&mut buff).is_err() {
            return Err("Error while Reading ~/.todo file!".to_string());
        }

        if buff.len() >= 1 {
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

        return Ok(Self { tasks, file });
    }

    fn add(&mut self, task: String) -> Result<(), String> {
        if self.tasks.contains(&task) {
            return Err("Task already exists!".to_string())
        }

        self.file.write_all(String::from(task.trim().to_owned() + "\n").as_bytes())
            .unwrap_or_else(|_e| { "Error while adding task!".to_string(); });

        return Ok(())
    }

    fn remove(&mut self, taskid: usize) -> Result<(), String> {
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
            self.file.write_all(String::from(task.trim().to_owned() + "\n").as_bytes())
                .unwrap_or_else(|_e| { "Error while adding task!".to_string(); });
        }

        return Ok(())
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(TODO_FILE)
        .expect("Failed to Open ~/.todo file!");

    let mut todo = Todo::init(file).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    match args.get(1).map(|s| s.as_str()) {
        Some("--add") => {
            if args.len() <= 2 {
                panic!("Empty task!?");
            }
            let _ = todo.add(String::from(args[2].clone())).map_err(|e| {
                panic!("{}", e);
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
                .unwrap_or_else(|_| {
                    panic!("Error while parsing task ID!");
                }));
        }
        _ => {
            let tasks_cell_width = todo.tasks
                .iter()
                .map(|s| s.len())
                .max()
                .unwrap_or(0);

            if tasks_cell_width == 0 {
                panic!("No Tasks!");
            }

            println!("┏━{}━┳━{}━┓", "━━━", "━".repeat(tasks_cell_width));
            println!("┃ {} ┃ {} ┃", "IDs", "Tasks".to_string() + &" ".repeat(
                if tasks_cell_width - 5 <= 0 { 0 } else { tasks_cell_width - 5 }
            ));
            println!("┣━{}━╋━{}━┫", "━━━", "━".repeat(tasks_cell_width));

            for i in 0..=todo.tasks.len() - 1 {
                let task = &todo.tasks[i];
                let task_cell_length = task.len();
                println!("┃ {:2}  ┃ {} ┃", i, task.to_owned() + &" ".repeat(tasks_cell_width - task_cell_length));
            }

            println!("┗━{}━┻━{}━┛", "━━━", "━".repeat(tasks_cell_width));
        }
    }
}
