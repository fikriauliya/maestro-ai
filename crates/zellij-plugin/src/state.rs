use zellij_tile::prelude::*;

use crate::process::ProcessInfo;

#[derive(Default, PartialEq, Clone, Copy)]
pub enum View {
    #[default]
    List,
    Detail,
}

#[derive(Default)]
pub struct State {
    pub processes: Vec<ProcessInfo>,
    pub selected_index: usize,
    pub view: View,
    pub loading: bool,
}

impl State {
    pub fn refresh_processes(&mut self) {
        self.loading = true;
        let mut context = std::collections::BTreeMap::new();
        context.insert("source".to_string(), "ps".to_string());
        run_command(&["ps", "aux", "--sort=-%cpu"], context);
    }

    pub fn set_processes(&mut self, processes: Vec<ProcessInfo>) {
        self.processes = processes;
        if self.selected_index >= self.processes.len() {
            self.selected_index = 0;
        }
    }

    pub fn handle_list_keys(&mut self, key: KeyWithModifier) -> bool {
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

    pub fn handle_detail_keys(&mut self, key: KeyWithModifier) -> bool {
        match key.bare_key {
            BareKey::Esc | BareKey::Backspace | BareKey::Char('q') => {
                self.view = View::List;
                true
            }
            _ => false,
        }
    }

    pub fn selected_process(&self) -> Option<&ProcessInfo> {
        self.processes.get(self.selected_index)
    }
}
