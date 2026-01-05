use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, StatefulWidget, Widget},
};

use crate::instance::InstanceStatus;
use crate::state::State;

pub fn render_loading(area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Claude Code ");

    let message = Paragraph::new("Loading instances...")
        .block(block)
        .centered();

    message.render(area, buf);
}

pub fn render_list(state: &State, area: Rect, buf: &mut Buffer) {
    if state.instances.is_empty() {
        render_empty(state, area, buf);
        return;
    }

    let items: Vec<ListItem> = state
        .instances
        .iter()
        .map(|instance| {
            let icon = instance.status.icon();
            let icon_style = match instance.status {
                InstanceStatus::Running => Style::default().fg(Color::Yellow),
                InstanceStatus::Waiting => Style::default().fg(Color::Cyan),
            };

            let line = Line::from(vec![
                Span::styled(icon, icon_style),
                Span::raw(" "),
                Span::raw(&instance.folder),
                Span::styled(
                    format!(" (pane {})", instance.pane_id),
                    Style::default().fg(Color::DarkGray),
                ),
            ]);
            ListItem::new(line)
        })
        .collect();

    let version_title = format_version(&state.version, &state.build);

    let title = format!(" Claude Code Instances ({}) ", state.instances.len());
    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .title_top(Line::from(version_title).right_aligned())
                .title_bottom(" ↑↓:Navigate  Enter:Switch  r:Refresh  q:Close "),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    let mut list_state = ListState::default();
    list_state.select(Some(state.selected_index));

    StatefulWidget::render(list, area, buf, &mut list_state);
}

fn render_empty(state: &State, area: Rect, buf: &mut Buffer) {
    let version_title = format_version(&state.version, &state.build);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Claude Code Instances ")
        .title_top(Line::from(version_title).right_aligned())
        .title_bottom(" q:Close ");

    let msg = if let Some(err) = &state.error {
        err.as_str()
    } else if state.version.is_empty() {
        "No data (maestro command failed?)"
    } else {
        "No Claude Code instances running"
    };

    let message = Paragraph::new(msg)
        .block(block)
        .centered();

    message.render(area, buf);
}

fn format_version(version: &str, build: &str) -> String {
    if version.is_empty() {
        String::new()
    } else {
        format!(" v{version} ({build}) ")
    }
}
