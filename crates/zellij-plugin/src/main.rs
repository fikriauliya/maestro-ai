mod instance;
mod state;
mod ui;

use zellij_tile::prelude::*;

use crate::instance::MaestroOutput;
use crate::state::State;

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: std::collections::BTreeMap<String, String>) {
        request_permission(&[
            PermissionType::RunCommands,
            PermissionType::ChangeApplicationState,
        ]);
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
                        let output = MaestroOutput::parse(&stdout);
                        self.set_instances(output.instances);
                    } else {
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
        ui::render(self, rows, cols);
    }
}
