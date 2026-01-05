use ratatui::{
    buffer::Buffer,
    style::{Color, Modifier, Style},
};

/// Render a ratatui Buffer to stdout using ANSI escape codes
pub fn render_buffer(buf: &Buffer) {
    let area = buf.area;

    for y in area.top()..area.bottom() {
        let mut line = String::new();
        let mut current_style: Option<Style> = None;

        for x in area.left()..area.right() {
            let cell = &buf[(x, y)];
            let style = cell.style();

            // Only emit style codes when style changes
            if current_style != Some(style) {
                // Reset previous style
                if current_style.is_some() {
                    line.push_str("\x1b[0m");
                }

                // Apply new style
                let mut codes = Vec::new();

                if style.fg.is_some() || style.bg.is_some() || !style.add_modifier.is_empty() {
                    if let Some(fg) = style.fg {
                        codes.push(fg_to_ansi(fg));
                    }
                    if let Some(bg) = style.bg {
                        codes.push(bg_to_ansi(bg));
                    }
                    if style.add_modifier.contains(Modifier::BOLD) {
                        codes.push("1".to_string());
                    }
                    if style.add_modifier.contains(Modifier::DIM) {
                        codes.push("2".to_string());
                    }
                    if style.add_modifier.contains(Modifier::ITALIC) {
                        codes.push("3".to_string());
                    }
                    if style.add_modifier.contains(Modifier::UNDERLINED) {
                        codes.push("4".to_string());
                    }
                    if style.add_modifier.contains(Modifier::REVERSED) {
                        codes.push("7".to_string());
                    }

                    if !codes.is_empty() {
                        line.push_str(&format!("\x1b[{}m", codes.join(";")));
                    }
                }

                current_style = Some(style);
            }

            line.push_str(cell.symbol());
        }

        // Reset at end of line
        if current_style.is_some() {
            line.push_str("\x1b[0m");
        }

        println!("{}", line);
    }
}

fn fg_to_ansi(color: Color) -> String {
    match color {
        Color::Black => "30".to_string(),
        Color::Red => "31".to_string(),
        Color::Green => "32".to_string(),
        Color::Yellow => "33".to_string(),
        Color::Blue => "34".to_string(),
        Color::Magenta => "35".to_string(),
        Color::Cyan => "36".to_string(),
        Color::Gray => "37".to_string(),
        Color::DarkGray => "90".to_string(),
        Color::LightRed => "91".to_string(),
        Color::LightGreen => "92".to_string(),
        Color::LightYellow => "93".to_string(),
        Color::LightBlue => "94".to_string(),
        Color::LightMagenta => "95".to_string(),
        Color::LightCyan => "96".to_string(),
        Color::White => "97".to_string(),
        Color::Rgb(r, g, b) => format!("38;2;{};{};{}", r, g, b),
        Color::Indexed(i) => format!("38;5;{}", i),
        Color::Reset => "39".to_string(),
    }
}

fn bg_to_ansi(color: Color) -> String {
    match color {
        Color::Black => "40".to_string(),
        Color::Red => "41".to_string(),
        Color::Green => "42".to_string(),
        Color::Yellow => "43".to_string(),
        Color::Blue => "44".to_string(),
        Color::Magenta => "45".to_string(),
        Color::Cyan => "46".to_string(),
        Color::Gray => "47".to_string(),
        Color::DarkGray => "100".to_string(),
        Color::LightRed => "101".to_string(),
        Color::LightGreen => "102".to_string(),
        Color::LightYellow => "103".to_string(),
        Color::LightBlue => "104".to_string(),
        Color::LightMagenta => "105".to_string(),
        Color::LightCyan => "106".to_string(),
        Color::White => "107".to_string(),
        Color::Rgb(r, g, b) => format!("48;2;{};{};{}", r, g, b),
        Color::Indexed(i) => format!("48;5;{}", i),
        Color::Reset => "49".to_string(),
    }
}
