use crate::instance::InstanceStatus;
use crate::state::State;

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const YELLOW: &str = "\x1b[33m";
const CYAN: &str = "\x1b[36m";
const BG_GRAY: &str = "\x1b[48;5;238m";

pub fn render(state: &State, _rows: usize, _cols: usize) {
    if state.loading {
        println!("Loading...");
        return;
    }

    if state.instances.is_empty() {
        println!("No instances");
        return;
    }

    for (i, instance) in state.instances.iter().enumerate() {
        let is_selected = i == state.selected_index;
        let icon = instance.status.icon();
        let icon_color = match instance.status {
            InstanceStatus::Running => YELLOW,
            InstanceStatus::Waiting => CYAN,
        };

        if is_selected {
            println!("{BG_GRAY}{BOLD}▶ {icon_color}{icon}{RESET}{BG_GRAY}{BOLD} {} {DIM}(pane {}){RESET}", instance.folder, instance.pane_id);
        } else {
            println!("  {icon_color}{icon}{RESET} {} {DIM}(pane {}){RESET}", instance.folder, instance.pane_id);
        }
    }
}
