use crate::model::{InputMode, Project, SortOrder, Task};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn draw_ui(
    f: &mut Frame,
    projects: &[Project],
    current_project: usize,
    current_column: usize,
    selected_task: usize,
    sort_order: SortOrder,
    input_mode: &InputMode, // FIXED: was `bool`, now matches what main.rs passes
    input_text: &str,
) {
    // Split screen into top bar, main area, and bottom input bar
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Project List & Sort Status
            Constraint::Min(10),   // Kanban Columns
            Constraint::Length(3), // Input Bar / Help text
        ])
        .split(f.area());

    // 1. Draw Top Project Bar
    let mut project_spans = vec![Span::raw("Projects: ")];
    for (i, proj) in projects.iter().enumerate() {
        let style = if i == current_project {
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Gray)
        };

        let time_str = proj.last_updated.format("%H:%M:%S").to_string();
        let text = format!(" {} [{}] ({}) ", proj.name, proj.total_tasks(), time_str);
        project_spans.push(Span::styled(text, style));
    }

    let sort_label = format!(" | Sort: {:?}", sort_order);
    project_spans.push(Span::styled(sort_label, Style::default().fg(Color::Magenta)));

    let top_bar = Paragraph::new(Line::from(project_spans))
        .block(Block::default().borders(Borders::ALL).title(" Kanban Dashboard "));
    f.render_widget(top_bar, chunks[0]);

    // 2. Draw Kanban Columns for the active project
    if let Some(project) = projects.get(current_project) {
        let columns_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(34),
            ])
            .split(chunks[1]);

        // Turn tasks into visual items, highlighting the selected row
        // when its column is the currently active one.
        let make_list_items = |tasks: &[Task], column_index: usize| -> Vec<ListItem> {
            tasks
                .iter()
                .enumerate()
                .map(|(i, t)| {
                    if column_index == current_column && i == selected_task {
                        ListItem::new(format!("> {}", t.title)).style(
                            Style::default()
                                .fg(Color::Black)
                                .bg(Color::White)
                                .add_modifier(Modifier::BOLD),
                        )
                    } else {
                        ListItem::new(format!("  {}", t.title))
                    }
                })
                .collect()
        };

        // Red Column: Not Done
        let not_done_style = if current_column == 0 {
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Red)
        };
        let not_done_block = Block::default()
            .borders(Borders::ALL)
            .title(" Not Done (Red) ")
            .border_style(not_done_style);
        let not_done_list = List::new(make_list_items(&project.not_done, 0)).block(not_done_block);
        f.render_widget(not_done_list, columns_chunks[0]);

        // Yellow Column: In Progress
        let in_progress_style = if current_column == 1 {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Yellow)
        };
        let in_progress_block = Block::default()
            .borders(Borders::ALL)
            .title(" In Progress (Yellow) ")
            .border_style(in_progress_style);
        let in_progress_list =
            List::new(make_list_items(&project.in_progress, 1)).block(in_progress_block);
        f.render_widget(in_progress_list, columns_chunks[1]);

        // Green Column: Done
        let done_style = if current_column == 2 {
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Green)
        };
        let done_block = Block::default()
            .borders(Borders::ALL)
            .title(" Done (Green) ")
            .border_style(done_style);
        let done_list = List::new(make_list_items(&project.done, 2)).block(done_block);
        f.render_widget(done_list, columns_chunks[2]);
    }

    // 3. Draw Bottom Input / Control Bar
    let bottom_text = if *input_mode != InputMode::Normal {
        let label = match input_mode {
            InputMode::AddTask => "task",
            InputMode::AddProject => "project",
            InputMode::Normal => "",
        };
        format!(" Adding {}: {}_", label, input_text)
    } else {
        " [t] add task  [p] add project  [d] delete project  [x] delete task  [[ / ]] move task  [Tab] column  [\u{2190}/\u{2192}] project  [\u{2191}/\u{2193}] task  [s] sort  [q] quit "
            .to_string()
    };

    let bottom_bar = Paragraph::new(bottom_text)
        .block(Block::default().borders(Borders::ALL).title(" Actions "));
    f.render_widget(bottom_bar, chunks[2]);
}