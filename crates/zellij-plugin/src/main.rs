use std::collections::BTreeMap;
use zellij_tile::prelude::*;

#[derive(Debug, Clone)]
struct ProcessInfo {
    pid: String,
    user: String,
    cpu: String,
    mem: String,
    vsz: String,
    rss: String,
    tty: String,
    stat: String,
    start: String,
    time: String,
    command: String,
}

#[derive(Default, PartialEq)]
enum View {
    #[default]
    List,
    Detail,
}

#[derive(Default)]
struct State {
    processes: Vec<ProcessInfo>,
    selected_index: usize,
    view: View,
    loading: bool,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        request_permission(&[PermissionType::RunCommands]);
        subscribe(&[EventType::Key, EventType::RunCommandResult, EventType::PermissionRequestResult]);
        self.loading = true;
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::PermissionRequestResult(PermissionStatus::Granted) => {
                self.refresh_processes();
                true
            }
            Event::RunCommandResult(exit_code, stdout, _stderr, context) => {
                if context.get("source").map(|s| s.as_str()) == Some("ps") {
                    if exit_code == Some(0) {
                        self.parse_ps_output(&stdout);
                    }
                    self.loading = false;
                }
                true
            }
            Event::Key(key) => {
                if self.view == View::List {
                    self.handle_list_keys(key)
                } else {
                    self.handle_detail_keys(key)
                }
            }
            _ => false,
        }
    }

    fn render(&mut self, rows: usize, cols: usize) {
        if self.loading {
            self.render_loading(rows, cols);
        } else if self.view == View::List {
            self.render_list(rows, cols);
        } else {
            self.render_detail(rows, cols);
        }
    }
}

impl State {
    fn refresh_processes(&mut self) {
        self.loading = true;
        let mut context = BTreeMap::new();
        context.insert("source".to_string(), "ps".to_string());
        run_command(&["ps", "aux", "--sort=-%cpu"], context);
    }

    fn parse_ps_output(&mut self, stdout: &[u8]) {
        let output = String::from_utf8_lossy(stdout);
        self.processes.clear();

        for (i, line) in output.lines().enumerate() {
            // Skip header line
            if i == 0 {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 11 {
                let process = ProcessInfo {
                    user: parts[0].to_string(),
                    pid: parts[1].to_string(),
                    cpu: parts[2].to_string(),
                    mem: parts[3].to_string(),
                    vsz: parts[4].to_string(),
                    rss: parts[5].to_string(),
                    tty: parts[6].to_string(),
                    stat: parts[7].to_string(),
                    start: parts[8].to_string(),
                    time: parts[9].to_string(),
                    command: parts[10..].join(" "),
                };
                self.processes.push(process);
            }
        }

        // Reset selection if out of bounds
        if self.selected_index >= self.processes.len() {
            self.selected_index = 0;
        }
    }

    fn handle_list_keys(&mut self, key: KeyWithModifier) -> bool {
        match key.bare_key {
            BareKey::Up | BareKey::Char('k') => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
                true
            }
            BareKey::Down | BareKey::Char('j') => {
                if self.selected_index < self.processes.len().saturating_sub(1) {
                    self.selected_index += 1;
                }
                true
            }
            BareKey::Enter => {
                if !self.processes.is_empty() {
                    self.view = View::Detail;
                }
                true
            }
            BareKey::Char('r') => {
                self.refresh_processes();
                true
            }
            BareKey::Esc | BareKey::Char('q') => {
                hide_self();
                true
            }
            _ => false,
        }
    }

    fn handle_detail_keys(&mut self, key: KeyWithModifier) -> bool {
        match key.bare_key {
            BareKey::Esc | BareKey::Backspace | BareKey::Char('q') => {
                self.view = View::List;
                true
            }
            _ => false,
        }
    }

    fn render_loading(&self, rows: usize, cols: usize) {
        let message = "Loading processes...";
        let row = rows / 2;
        let col = cols.saturating_sub(message.len()) / 2;

        for _ in 0..row {
            println!();
        }
        println!("{:>width$}", message, width = col + message.len());
    }

    fn render_list(&self, rows: usize, cols: usize) {
        // Header
        let title = format!(
            " Process List ({} processes) - ↑↓/jk:Navigate  Enter:Details  r:Refresh  q:Quit ",
            self.processes.len()
        );
        println!(
            "\x1b[1;7m{:^width$}\x1b[0m",
            title,
            width = cols
        );
        println!();

        // Column headers
        let header = format!(
            "{:<8} {:<12} {:>6} {:>6} {:<}",
            "PID", "USER", "CPU%", "MEM%", "COMMAND"
        );
        println!("\x1b[1;4m{}\x1b[0m", header);

        // Calculate visible range (accounting for header lines)
        let visible_rows = rows.saturating_sub(4);
        let start = if self.selected_index >= visible_rows {
            self.selected_index - visible_rows + 1
        } else {
            0
        };
        let end = (start + visible_rows).min(self.processes.len());

        // Render process list
        for (i, process) in self.processes.iter().enumerate().skip(start).take(end - start) {
            let cmd_width = cols.saturating_sub(36);
            let cmd_display: String = if process.command.len() > cmd_width {
                format!("{}...", &process.command[..cmd_width.saturating_sub(3)])
            } else {
                process.command.clone()
            };

            let line = format!(
                "{:<8} {:<12} {:>6} {:>6} {}",
                process.pid,
                truncate_str(&process.user, 12),
                process.cpu,
                process.mem,
                cmd_display
            );

            if i == self.selected_index {
                println!("\x1b[7m{:<width$}\x1b[0m", line, width = cols);
            } else {
                println!("{}", line);
            }
        }
    }

    fn render_detail(&self, rows: usize, cols: usize) {
        let Some(process) = self.processes.get(self.selected_index) else {
            println!("No process selected");
            return;
        };

        // Header
        let title = " Process Details - Esc/q:Back ";
        println!(
            "\x1b[1;7m{:^width$}\x1b[0m",
            title,
            width = cols
        );
        println!();

        // Process details
        let details = [
            ("PID", &process.pid),
            ("User", &process.user),
            ("CPU %", &process.cpu),
            ("Memory %", &process.mem),
            ("VSZ (KB)", &process.vsz),
            ("RSS (KB)", &process.rss),
            ("TTY", &process.tty),
            ("Status", &process.stat),
            ("Started", &process.start),
            ("CPU Time", &process.time),
        ];

        for (label, value) in details {
            println!("\x1b[1m{:<12}\x1b[0m: {}", label, value);
        }

        println!();
        println!("\x1b[1mCommand:\x1b[0m");

        // Word wrap the command
        let cmd = &process.command;
        let wrap_width = cols.saturating_sub(2);
        let mut remaining = cmd.as_str();
        while !remaining.is_empty() {
            let (line, rest) = if remaining.len() > wrap_width {
                remaining.split_at(wrap_width)
            } else {
                (remaining, "")
            };
            println!("  {}", line);
            remaining = rest;
        }

        // Status explanation
        println!();
        println!("\x1b[1mStatus Legend:\x1b[0m");
        println!("  R=Running  S=Sleeping  D=Disk Wait  Z=Zombie  T=Stopped");

        // Fill remaining space
        let used_rows = 16 + (cmd.len() / wrap_width.max(1)) + 1;
        for _ in used_rows..rows {
            println!();
        }
    }
}

fn truncate_str(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    } else {
        s.to_string()
    }
}
