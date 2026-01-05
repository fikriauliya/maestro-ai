mod ansi;
mod process;
mod state;
mod ui;

use ratatui::{buffer::Buffer, layout::Rect};
use zellij_tile::prelude::*;

use crate::process::ProcessInfo;
use crate::state::{State, View};

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: std::collections::BTreeMap<String, String>) {
        request_permission(&[PermissionType::RunCommands]);
        subscribe(&[
            EventType::Key,
            EventType::RunCommandResult,
            EventType::PermissionRequestResult,
        ]);
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
                        let processes = ProcessInfo::parse_ps_output(&stdout);
                        self.set_processes(processes);
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
        let area = Rect::new(0, 0, cols as u16, rows as u16);
        let mut buf = Buffer::empty(area);

        if self.loading {
            ui::render_loading(area, &mut buf);
        } else if self.view == View::List {
            ui::render_list(self, area, &mut buf);
        } else {
            ui::render_detail(self, area, &mut buf);
        }

        ansi::render_buffer(&buf);
    }
}
