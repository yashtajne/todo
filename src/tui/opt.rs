
use std::fs::File;
use crate::tui::status::Status;

pub enum Mode { Normal, Insert }

pub struct Task {
    pub task:    String,
    pub status:  Status
}

pub struct Todo {
    pub tasks: Vec<Task>,
    pub file:  File,
    pub tasks_cell_width: usize
}

pub struct ListOptions {
    pub mode: Mode,
    pub cur:  Option<usize>
}


