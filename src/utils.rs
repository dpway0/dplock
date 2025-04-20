use std::io;
use std::io::Write;
use anyhow::anyhow;
use base64::Engine;
use chrono::Utc;
use serde_json::Value;
use terminal_size::terminal_size;
use base64::engine::general_purpose::STANDARD as base64_engine;

pub fn get_terminal_width() -> usize {
    if let Some((w, _)) = terminal_size() {
        w.0 as usize
    } else {
        80 // fallback
    }
}

pub fn is_encrypted(data: &[u8]) -> bool {
    let json: Value = match serde_json::from_slice(data) {
        Ok(v) => v,
        Err(_) => return false,
    };

    let entries = match json.get("entries").and_then(|e| e.as_object()) {
        Some(map) => map,
        None => return false,
    };

    for entry_list in entries.values() {
        if let Some(array) = entry_list.as_array() {
            for entry in array {
                if let Some(password_value) = entry.get("password") {
                    if let Some(password) = password_value.as_str() {
                        if let Ok(decoded) = base64_engine.decode(password) {
                            if decoded.len() >= 32 {
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }

    false
}



pub fn parse_expired_time(input: &str) -> anyhow::Result<i64> {
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
        Err(anyhow!("❌ Invalid expired time format. Use e.g: 3h, 1d, 2w, 3m, 4y"))
    } else {
        Ok(now + total_secs)
    }
}

pub fn parse_remind_time(expired: Option<i64>, input: &str) -> anyhow::Result<i64> {
    let expired_secs = expired.unwrap_or(0);
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
        Err(anyhow!("❌ Invalid remind time format. Use e.g: 3h, 1d, 2w, 3m, 4y"))
    } else if expired_secs == 0 {
        Err(anyhow!("❌ Invalid expired time format. Use e.g: 3h, 1d, 2w, 3m, 4y"))
    }
    else {
        Ok(expired_secs - total_secs)
    }
}

pub fn compute_wait_time(attempts: u32) -> u64 {
        2u64.pow((attempts - 5).min(5))
    }
    
pub fn wait_with_countdown(wait_minutes: u64) -> anyhow::Result<()> {
        for remaining in (1..=wait_minutes * 60).rev() {
            print!("\r⏳ Please wait {} second(s)...", remaining);
            io::stdout().flush()?;
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
        println!();
        Ok(())
    }