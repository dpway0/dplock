use terminal_size::{terminal_size, Width};

pub fn get_terminal_width() -> usize {
    if let Some((Width(w), _)) = terminal_size() {
        w as usize
    } else {
        80 // fallback
    }
}