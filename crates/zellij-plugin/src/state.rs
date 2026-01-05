use zellij_tile::prelude::*;

use crate::instance::{ClaudeInstance, MaestroOutput};

#[derive(Default)]
pub struct State {
    pub instances: Vec<ClaudeInstance>,
    pub selected_index: usize,
    pub loading: bool,
    pub version: String,
    pub build: String,
    pub error: Option<String>,
}

impl State {
    const MAESTRO_BIN: &str = "/home/levifikri/.cargo/bin/maestro";

    pub fn refresh_instances(&mut self) {
        self.loading = true;
        let mut context = std::collections::BTreeMap::new();
        context.insert("source".to_string(), "instances".to_string());
        run_command(&[Self::MAESTRO_BIN, "list", "--json"], context);
    }

    pub fn set_from_output(&mut self, output: MaestroOutput) {
        self.version = output.version;
        self.build = output.build;
        self.instances = output.instances;
        if self.selected_index >= self.instances.len() && !self.instances.is_empty() {
            self.selected_index = self.instances.len() - 1;
        }
    }

    pub fn handle_keys(&mut self, key: KeyWithModifier) -> bool {
        match key.bare_key {
            BareKey::Up | BareKey::Char('k') => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
                true
            }
            BareKey::Down | BareKey::Char('j') => {
                if self.selected_index < self.instances.len().saturating_sub(1) {
                    self.selected_index += 1;
                }
                true
            }
            BareKey::Enter => {
                self.focus_selected_instance();
                true
            }
            BareKey::Char('r') => {
                self.refresh_instances();
                true
            }
            BareKey::Esc | BareKey::Char('q') => {
                hide_self();
                true
            }
            _ => false,
        }
    }

    pub fn focus_selected_instance(&self) {
        if let Some(instance) = self.instances.get(self.selected_index) {
            focus_terminal_pane(instance.pane_id, true);
            hide_self();
        }
    }

}
