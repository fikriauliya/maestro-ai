mod ansi;
mod instance;
mod state;
mod ui;

use ratatui::{buffer::Buffer, layout::Rect};
use zellij_tile::prelude::*;

use crate::instance::ClaudeInstance;
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
            Event::RunCommandResult(exit_code, stdout, _stderr, context) => {
                if context.get("source").map(|s| s.as_str()) == Some("instances") {
                    if exit_code == Some(0) {
                        let instances = ClaudeInstance::parse_json(&stdout);
                        self.set_instances(instances);
                    } else {
                        // File doesn't exist or error - clear instances
                        self.set_instances(Vec::new());
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
