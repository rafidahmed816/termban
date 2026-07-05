use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

mod model;
mod storage;
mod ui;

use model::{sort_projects, InputMode, Project, SortOrder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Load whatever was saved last time; if there's nothing saved yet (or
    // loading fails for some reason) start fresh with one empty project.
    let mut projects = storage::load_projects().unwrap_or_default();
    if projects.is_empty() {
        projects.push(Project::new("My First Project"));
    }

    let mut current_project = 0;
    let mut current_column = 0;
    let mut selected_task = 0; // NEW: which task is highlighted in the current column
    let mut sort_order = SortOrder::Alphabetical;

    let mut input_mode = InputMode::Normal;
    let mut input_text = String::new();

    loop {
        sort_projects(&mut projects, sort_order);

        // Keep the selected task in bounds any time the column/project changes
        // size (e.g. right after a delete, or after switching columns/projects).
        if let Some(proj) = projects.get(current_project) {
            let len = proj.get_column(current_column).len();
            if len == 0 {
                selected_task = 0;
            } else if selected_task >= len {
                selected_task = len - 1;
            }
        } else {
            selected_task = 0;
        }

        terminal.draw(|f| {
            ui::draw_ui(
                f,
                &projects,
                current_project,
                current_column,
                selected_task,
                sort_order,
                &input_mode,
                &input_text,
            )
        })?;

        if let Event::Key(key) = event::read()? {
            if input_mode != InputMode::Normal {
                // We are typing text for a task or a project
                match key.code {
                    KeyCode::Enter => {
                        if !input_text.is_empty() {
                            match input_mode {
                                InputMode::AddTask => {
                                    if let Some(proj) = projects.get_mut(current_project) {
                                        proj.add_task(current_column, input_text.clone());
                                    }
                                }
                                InputMode::AddProject => {
                                    projects.push(Project::new(&input_text));
                                    current_project = projects.len() - 1; // Jump to new project
                                }
                                _ => {}
                            }
                            input_text.clear();
                            let _ = storage::save_projects(&projects);
                        }
                        input_mode = InputMode::Normal;
                    }
                    KeyCode::Esc => {
                        input_mode = InputMode::Normal;
                        input_text.clear();
                    }
                    KeyCode::Char(c) => {
                        input_text.push(c);
                    }
                    KeyCode::Backspace => {
                        input_text.pop();
                    }
                    _ => {}
                }
            } else {
                // Normal mode controls
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Tab => {
                        current_column = (current_column + 1) % 3;
                        selected_task = 0;
                    }
                    KeyCode::Left => {
                        if current_project > 0 {
                            current_project -= 1;
                        } else if !projects.is_empty() {
                            current_project = projects.len() - 1;
                        }
                        selected_task = 0;
                    }
                    KeyCode::Right => {
                        if !projects.is_empty() {
                            current_project = (current_project + 1) % projects.len();
                        }
                        selected_task = 0;
                    }
                    // NEW: move the task cursor up/down within the current column
                    KeyCode::Up => {
                        if selected_task > 0 {
                            selected_task -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if let Some(proj) = projects.get(current_project) {
                            let len = proj.get_column(current_column).len();
                            if len > 0 && selected_task + 1 < len {
                                selected_task += 1;
                            }
                        }
                    }
                    // Press 't' to add a task
                    KeyCode::Char('t') => {
                        if !projects.is_empty() {
                            input_mode = InputMode::AddTask;
                        }
                    }
                    // Press 'p' to add a new project
                    KeyCode::Char('p') => {
                        input_mode = InputMode::AddProject;
                    }
                    // Press 'd' to delete current project
                    KeyCode::Char('d') => {
                        if !projects.is_empty() {
                            projects.remove(current_project);
                            // Fix the index if we deleted the last item
                            if current_project >= projects.len() && current_project > 0 {
                                current_project -= 1;
                            }
                            selected_task = 0;
                            let _ = storage::save_projects(&projects);
                        }
                    }
                    // NEW: Press '[' to move the highlighted task to the previous column
                    KeyCode::Char('[') => {
                        if current_column > 0 {
                            if let Some(proj) = projects.get_mut(current_project) {
                                if let Some(new_index) =
                                    proj.move_task(current_column, selected_task, current_column - 1)
                                {
                                    current_column -= 1;
                                    selected_task = new_index;
                                    let _ = storage::save_projects(&projects);
                                }
                            }
                        }
                    }
                    // NEW: Press ']' to move the highlighted task to the next column
                    KeyCode::Char(']') => {
                        if current_column < 2 {
                            if let Some(proj) = projects.get_mut(current_project) {
                                if let Some(new_index) =
                                    proj.move_task(current_column, selected_task, current_column + 1)
                                {
                                    current_column += 1;
                                    selected_task = new_index;
                                    let _ = storage::save_projects(&projects);
                                }
                            }
                        }
                    }
                    // Press 'x' to delete the highlighted task in the current column
                    KeyCode::Char('x') => {
                        if let Some(proj) = projects.get_mut(current_project) {
                            proj.delete_task(current_column, selected_task);
                            if selected_task > 0 {
                                selected_task -= 1;
                            }
                            let _ = storage::save_projects(&projects);
                        }
                    }
                    KeyCode::Char('s') => {
                        sort_order = match sort_order {
                            SortOrder::Alphabetical => SortOrder::TimeUpdated,
                            SortOrder::TimeUpdated => SortOrder::TaskCount,
                            SortOrder::TaskCount => SortOrder::Alphabetical,
                        };
                    }
                    _ => {}
                }
            }
        }
    }

    let _ = storage::save_projects(&projects);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}