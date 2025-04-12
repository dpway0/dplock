use terminal_size::terminal_size;

pub fn get_terminal_width() -> usize {
    if let Some((w, _)) = terminal_size() {
        w.0 as usize
    } else {
        80 // fallback
    }
}

pub fn is_encrypted(data: &[u8]) -> bool {
    data.len() > 16 + 12
}