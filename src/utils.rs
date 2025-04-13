use anyhow::anyhow;
use chrono::Utc;
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

pub fn parse_relative_time(input: &str) -> anyhow::Result<i64> {
    let now = Utc::now().timestamp();
    let trimmed = input.trim().to_lowercase();
    let re = regex::Regex::new(r"(\d+)([hdwmy])")?;
    let mut total_secs = 0;

    for cap in re.captures_iter(&trimmed) {
        let num: i64 = cap[1].parse()?;
        let unit = &cap[2];
        total_secs += match unit {
            "h" => num * 3600,
            "d" => num * 86400,
            "w" => num * 7 * 86400,
            "m" => num * 30 * 86400,
            "y" => num * 365 * 86400,
            _ => 0,
        };
    }

    if total_secs == 0 {
        Err(anyhow!("❌ Invalid time format. Use e.g: 3h, 1d, 2w, 3m, 4y"))
    } else {
        Ok(now + total_secs)
    }
}