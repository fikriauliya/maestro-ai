use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, StatefulWidget, Widget, Wrap},
};

use crate::state::State;

pub fn render_loading(area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Process Viewer ");

    let message = Paragraph::new("Loading processes...")
        .block(block)
        .centered();

    message.render(area, buf);
}

pub fn render_list(state: &State, area: Rect, buf: &mut Buffer) {
    let chunks = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(area);

    // Header
    let title = format!(
        " Process List ({}) │ ↑↓/jk:Navigate  Enter:Details  r:Refresh  q:Quit ",
        state.processes.len()
    );
    let header = Paragraph::new(title)
        .block(Block::default().borders(Borders::ALL))
        .centered();
    header.render(chunks[0], buf);

    // Process list
    let items: Vec<ListItem> = state
        .processes
        .iter()
        .map(|p| {
            let cmd_width = area.width.saturating_sub(40) as usize;
            let cmd = if p.command.len() > cmd_width {
                format!("{}…", &p.command[..cmd_width.saturating_sub(1)])
            } else {
                p.command.clone()
            };

            let line = Line::from(vec![
                Span::styled(format!("{:<8}", p.pid), Style::default().fg(Color::Cyan)),
                Span::raw(" "),
                Span::styled(format!("{:<10}", truncate(&p.user, 10)), Style::default()),
                Span::raw(" "),
                Span::styled(
                    format!("{:>5}", p.cpu),
                    if p.cpu.parse::<f32>().unwrap_or(0.0) > 10.0 {
                        Style::default().fg(Color::Red)
                    } else {
                        Style::default().fg(Color::Green)
                    },
                ),
                Span::raw(" "),
                Span::styled(format!("{:>5}", p.mem), Style::default().fg(Color::Yellow)),
                Span::raw("  "),
                Span::raw(cmd),
            ]);
            ListItem::new(line)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" PID      USER       CPU%  MEM%  COMMAND "),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    let mut list_state = ListState::default();
    list_state.select(Some(state.selected_index));

    StatefulWidget::render(list, chunks[1], buf, &mut list_state);
}

pub fn render_detail(state: &State, area: Rect, buf: &mut Buffer) {
    let Some(process) = state.selected_process() else {
        Paragraph::new("No process selected").render(area, buf);
        return;
    };

    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(12),
        Constraint::Min(3),
        Constraint::Length(3),
    ])
    .split(area);

    // Header
    let header = Paragraph::new(format!(" Process {} │ Esc/q:Back ", process.pid))
        .block(Block::default().borders(Borders::ALL))
        .centered();
    header.render(chunks[0], buf);

    // Details
    let details = vec![
        Line::from(vec![
            Span::styled("PID:        ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(&process.pid),
        ]),
        Line::from(vec![
            Span::styled("User:       ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(&process.user),
        ]),
        Line::from(vec![
            Span::styled("CPU %:      ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(&process.cpu, Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::styled("Memory %:   ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(&process.mem, Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::styled("VSZ (KB):   ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(&process.vsz),
        ]),
        Line::from(vec![
            Span::styled("RSS (KB):   ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(&process.rss),
        ]),
        Line::from(vec![
            Span::styled("TTY:        ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(&process.tty),
        ]),
        Line::from(vec![
            Span::styled("Status:     ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(
                &process.stat,
                Style::default().fg(match process.stat.chars().next() {
                    Some('R') => Color::Green,
                    Some('S') => Color::Blue,
                    Some('Z') => Color::Red,
                    _ => Color::White,
                }),
            ),
        ]),
        Line::from(vec![
            Span::styled("Started:    ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(&process.start),
        ]),
        Line::from(vec![
            Span::styled("CPU Time:   ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(&process.time),
        ]),
    ];

    let details_widget = Paragraph::new(details)
        .block(Block::default().borders(Borders::ALL).title(" Details "));
    details_widget.render(chunks[1], buf);

    // Command
    let cmd_widget = Paragraph::new(process.command.as_str())
        .block(Block::default().borders(Borders::ALL).title(" Command "))
        .wrap(Wrap { trim: false });
    cmd_widget.render(chunks[2], buf);

    // Status legend
    let legend = Paragraph::new("R=Running  S=Sleeping  D=Disk Wait  Z=Zombie  T=Stopped")
        .block(Block::default().borders(Borders::ALL).title(" Legend "))
        .centered();
    legend.render(chunks[3], buf);
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() > max {
        format!("{}…", &s[..max.saturating_sub(1)])
    } else {
        s.to_string()
    }
}
