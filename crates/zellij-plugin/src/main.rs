mod ansi;
mod instance;
mod state;
mod ui;

use ratatui::{buffer::Buffer, layout::Rect};
use zellij_tile::prelude::*;

use crate::instance::MaestroOutput;
use crate::state::State;

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: std::collections::BTreeMap<String, String>) {
        request_permission(&[PermissionType::RunCommands]);
        subscribe(&[
            EventType::Key,
            EventType::Timer,
            EventType::RunCommandResult,
            EventType::PermissionRequestResult,
        ]);
        self.loading = true;
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::PermissionRequestResult(PermissionStatus::Granted) => {
                self.refresh_instances();
                set_timeout(1.0);
                true
            }
            Event::Timer(_) => {
                self.refresh_instances();
                set_timeout(1.0);
                true
            }
            Event::RunCommandResult(exit_code, stdout, stderr, context) => {
                if context.get("source").map(|s| s.as_str()) == Some("instances") {
                    if exit_code == Some(0) {
                        let output = MaestroOutput::parse(&stdout);
                        self.set_from_output(output);
                        self.error = None;
                    } else {
                        // Command failed - show error
                        let err = String::from_utf8_lossy(&stderr).to_string();
                        self.error = Some(format!("exit {:?}: {}", exit_code, err));
                        self.set_from_output(MaestroOutput::default());
                    }
                    self.loading = false;
                }
                true
            }
            Event::Key(key) => self.handle_keys(key),
            _ => false,
        }
    }

    fn render(&mut self, rows: usize, cols: usize) {
        let area = Rect::new(0, 0, cols as u16, rows as u16);
        let mut buf = Buffer::empty(area);

        if self.loading {
            ui::render_loading(area, &mut buf);
        } else {
            ui::render_list(self, area, &mut buf);
        }

        ansi::render_buffer(&buf);
    }
}
