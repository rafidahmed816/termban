use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    pub title: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub not_done: Vec<Task>,
    pub in_progress: Vec<Task>,
    pub done: Vec<Task>,
    pub last_updated: DateTime<Local>,
}

impl Project {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            not_done: Vec::new(),
            in_progress: Vec::new(),
            done: Vec::new(),
            last_updated: Local::now(),
        }
    }

    pub fn total_tasks(&self) -> usize {
        self.not_done.len() + self.in_progress.len() + self.done.len()
    }

    pub fn add_task(&mut self, column_index: usize, title: String) {
        let task = Task { title };
        match column_index {
            0 => self.not_done.push(task),
            1 => self.in_progress.push(task),
            2 => self.done.push(task),
            _ => {}
        }
        self.last_updated = Local::now();
    }

    /// Read-only access to a column by index (0 = not_done, 1 = in_progress, 2 = done).
    pub fn get_column(&self, column_index: usize) -> &Vec<Task> {
        match column_index {
            0 => &self.not_done,
            1 => &self.in_progress,
            _ => &self.done,
        }
    }

    /// Mutable access to a column by index (0 = not_done, 1 = in_progress, 2 = done).
    fn get_column_mut(&mut self, column_index: usize) -> &mut Vec<Task> {
        match column_index {
            0 => &mut self.not_done,
            1 => &mut self.in_progress,
            _ => &mut self.done,
        }
    }

    /// Removes the task at `task_index` within the given column, if it exists.
    pub fn delete_task(&mut self, column_index: usize, task_index: usize) {
        let column = self.get_column_mut(column_index);
        if task_index < column.len() {
            column.remove(task_index);
            self.last_updated = Local::now();
        }
    }

    /// Moves the task at `task_index` in `from_column` to the end of `to_column`.
    /// Returns the task's new index within `to_column` on success, or `None`
    /// if the columns are the same or the index is out of range.
    pub fn move_task(
        &mut self,
        from_column: usize,
        task_index: usize,
        to_column: usize,
    ) -> Option<usize> {
        if from_column == to_column {
            return None;
        }
        let task = {
            let source = self.get_column_mut(from_column);
            if task_index >= source.len() {
                return None;
            }
            source.remove(task_index)
        };
        let new_index = {
            let dest = self.get_column_mut(to_column);
            dest.push(task);
            dest.len() - 1
        };
        self.last_updated = Local::now();
        Some(new_index)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortOrder {
    Alphabetical,
    TimeUpdated,
    TaskCount,
}

pub fn sort_projects(projects: &mut [Project], order: SortOrder) {
    match order {
        SortOrder::Alphabetical => {
            projects.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        }
        SortOrder::TimeUpdated => projects.sort_by(|a, b| b.last_updated.cmp(&a.last_updated)),
        SortOrder::TaskCount => projects.sort_by(|a, b| b.total_tasks().cmp(&a.total_tasks())),
    }
}

/// Tracks what the user is currently typing.
#[derive(Debug, PartialEq)]
pub enum InputMode {
    Normal,
    AddTask,
    AddProject,
}