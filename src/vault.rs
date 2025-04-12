use crate::crypto;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};
use anyhow::{anyhow, Result};
use arboard::Clipboard;
use rpassword::prompt_password;
use std::io::{self, Write};
use crossterm::event::{Event, KeyCode};
use crossterm::{event};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use textwrap::wrap;
use crate::utils::get_terminal_width;

#[derive(Serialize, Deserialize)]
pub struct Entry {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct VaultData {
    pub entries: HashMap<String, Vec<Entry>>,
}

pub struct Vault;

impl Vault {
    fn vault_path() -> PathBuf {
        dirs::home_dir().unwrap().join(".dplock/vault.bin")
    }

    fn prompt_master_password(prompt: &str) -> Result<String> {
        prompt_password(prompt).map_err(|e| anyhow!("Failed to read password: {e}"))
    }

    fn load_vault(password: &str) -> Result<VaultData> {
        let bytes = fs::read(Self::vault_path())?;
        crypto::decrypt(&bytes, password)
    }

    pub fn init() -> Result<()> {
        let path = Self::vault_path();
        if path.exists() {
            println!("⚠️  Vault already exists at: {}", path.display());
            if !Self::confirm_overwrite()? {
                println!("❌ Initialization cancelled.");
                return Ok(());
            }
        }

        let new_password = prompt_password("Set new master password: ")?;
        Self::save_vault(&VaultData::default(), &new_password)?;
        println!("🔐 Vault initialized!");
        Ok(())
    }

    pub fn add(name: &str, username: &str) -> Result<()> {
        let master = Self::prompt_master_password("🔐 Master password: ")?;
        let entry_pass = Self::prompt_master_password("🔑 Entry password: ")?;
        let mut data = Self::load_vault(&master)?;

        let entry = Entry {
            username: username.to_string(),
            password: entry_pass,
        };

        data.entries.entry(name.to_string()).or_default().push(entry);

        Self::save_vault(&data, &master)?;
        println!("✅ Entry added under: {}", name);
        Ok(())
    }

    pub fn get(name: &str, show: bool) -> Result<()> {
        let password = Self::prompt_master_password("Master password: ")?;
        let data = Self::load_vault(&password)?;

        if let Some(entries) = data.entries.get(name) {
            println!("🔐 Found {} entr{} for: {}", entries.len(), if entries.len() > 1 { "ies" } else { "y" }, name);
            for (i, entry) in entries.iter().enumerate() {
                println!("{}. 👤 Username: {}", i + 1, entry.username);
                if show {
                    println!("   🔑 Password: {}", entry.password);
                } else {
                    Self::copy_to_clipboard(&entry.password)?;
                    println!("   📋 Password copied to clipboard!");
                }
            }
        } else {
            println!("❌ Entry not found");
        }

        Ok(())
    }

    pub fn list(filter: Option<&str>, sort: Option<&str>) -> Result<()> {
        let password = prompt_password("Master password: ")?;
        let data = Self::load(&password)?;

        let mut entries: Vec<_> = data.entries.iter().flat_map(|(name, entry_list)| {
            entry_list.iter().map(move |entry| (name, entry))
        }).collect();

        Self::apply_filter_and_sort(&mut entries, filter, sort);

        if entries.is_empty() {
            println!("📭 No matching entries found.");
            return Ok(());
        }

        println!("📒 Entries:");
        Self::paginate_entries(entries)?;
        Ok(())
    }

    pub fn remove(name: &str, index: Option<usize>) -> Result<()> {
        let master = Self::prompt_master_password("🔐 Master password: ")?;
        let mut data = Self::load_vault(&master)?;

        match data.entries.get_mut(name) {
            Some(entry_list) => {
                if let Some(idx) = index {
                    if idx == 0 || idx > entry_list.len() {
                        println!("❌ Invalid index. Use: 1..{}.", entry_list.len());
                        return Ok(());
                    }
                    let removed = entry_list.remove(idx - 1);
                    println!("🗑️ Removed entry: {} (👤 {})", name, removed.username);

                    if entry_list.is_empty() {
                        data.entries.remove(name);
                    }
                } else {
                    // Cảnh báo khi xóa toàn bộ entries
                    println!("⚠️  This will remove ALL {} entr{} under '{}'.",
                             entry_list.len(),
                             if entry_list.len() > 1 { "ies" } else { "y" },
                             name);

                    let confirm = Self::prompt_master_password("Type 'yes' to confirm: ")?;
                    if confirm.trim() != "yes" {
                        println!("❌ Cancelled.");
                        return Ok(());
                    }

                    data.entries.remove(name);
                    println!("🗑️ All entries under '{}' removed.", name);
                }

                Self::save_vault(&data, &master)?;
            }
            None => {
                println!("❌ Entry name not found.");
            }
        }

        Ok(())
    }


    fn load(password: &str) -> Result<VaultData> {
        let bytes = fs::read(Self::vault_path())?;
        let data: VaultData = crypto::decrypt(&bytes, password)?;
        Ok(data)
    }

    fn save_vault(data: &VaultData, password: &str) -> Result<()> {
        let encrypted = crypto::encrypt(data, password)?;
        fs::create_dir_all(Self::vault_path().parent().unwrap())?;
        fs::write(Self::vault_path(), encrypted)?;
        Ok(())
    }

    fn confirm_overwrite() -> Result<bool> {
        let confirm = prompt_password("Do you want to overwrite it? Type 'yes' to confirm: ")?;
        if confirm.trim() != "yes" {
            return Ok(false);
        }

        let old_password = prompt_password("Enter current master password: ")?;
        match Self::load(&old_password) {
            Ok(_) => {
                println!("✅ Password confirmed.");
                Ok(true)
            }
            Err(_) => {
                println!("❌ Wrong master password. Vault not overwritten.");
                Ok(false)
            }
        }
    }

    fn copy_to_clipboard(text: &str) -> Result<()> {
        let mut clipboard = Clipboard::new().map_err(|e| anyhow!("Clipboard error: {e}"))?;
        clipboard
            .set_text(text.to_string())
            .map_err(|e| anyhow!("Failed to copy to clipboard: {e}"))?;
        Ok(())
    }

    fn apply_filter_and_sort(
        entries: &mut Vec<(&String, &Entry)>,
        filter: Option<&str>,
        sort: Option<&str>,
    ) {
        if let Some(f) = filter {
            entries.retain(|(name, entry)| name.contains(f) || entry.username.contains(f));
        }

        if let Some(sort_key) = sort {
            match sort_key {
                "name" => entries.sort_by_key(|(name, _)| *name),
                "username" => entries.sort_by_key(|(_, entry)| &entry.username),
                _ => {}
            }
        }
    }

    fn paginate_entries(entries: Vec<(&String, &Entry)>) -> Result<()> {
        let term_width = get_terminal_width().saturating_sub(2);
        let page_size = 3;

        for (i, (name, entry)) in entries.iter().enumerate() {
            let line = format!("• {} (👤 {})", name, entry.username);
            for wrapped in wrap(&line, term_width) {
                println!("{}", wrapped);
            }

            if (i + 1) % page_size == 0 && i + 1 < entries.len() {
                print!("-- More (press any key to continue, q to quit) -- ");
                io::stdout().flush()?;

                enable_raw_mode()?;
                let should_quit = if let Event::Key(key_event) = event::read()? {
                    matches!(key_event.code, KeyCode::Char('q'))
                } else {
                    false
                };
                disable_raw_mode()?;

                if should_quit {
                    println!();
                    break;
                }

                print!("\r{}\r", " ".repeat(60));
                io::stdout().flush()?;
            }
        }

        Ok(())
    }
}
