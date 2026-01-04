use std::collections::BTreeMap;
use zellij_tile::prelude::*;

#[derive(Default)]
struct State;

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        subscribe(&[EventType::Key]);
    }

    fn update(&mut self, event: Event) -> bool {
        if let Event::Key(key) = event {
            if key.bare_key == BareKey::Esc {
                hide_self();
            }
        }
        true
    }

    fn render(&mut self, rows: usize, cols: usize) {
        let message = "Hello, World!";
        let row = rows / 2;
        let col = cols.saturating_sub(message.len()) / 2;

        for _ in 0..row {
            println!();
        }
        println!("{:>width$}", message, width = col + message.len());
    }
}
