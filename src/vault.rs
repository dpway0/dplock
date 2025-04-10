use crate::crypto;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};
use anyhow::Result;
use rpassword::prompt_password;

#[derive(Serialize, Deserialize)]
pub struct Entry {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct VaultData {
    pub entries: HashMap<String, Entry>,
}

pub struct Vault;

impl Vault {
    fn vault_path() -> PathBuf {
        dirs::home_dir().unwrap().join(".dplock/vault.bin")
    }

    pub fn init() -> Result<()> {
        let password = prompt_password("Master password: ")?;
        let data = VaultData::default();
        let encrypted = crypto::encrypt(&data, &password)?;
        fs::create_dir_all(Self::vault_path().parent().unwrap())?;
        fs::write(Self::vault_path(), encrypted)?;
        println!("🔐 Vault initialized!");
        Ok(())
    }
pub fn add(name: &str, username: &str, password_val: &str) -> Result<()> {
    let password = prompt_password("Master password: ")?;
    let mut data = Self::load(&password)?;
    data.entries.insert(
        name.to_string(),
        Entry {
            username: username.to_string(),
            password: password_val.to_string(),
        },
    );
    let encrypted = crypto::encrypt(&data, &password)?;
    fs::write(Self::vault_path(), encrypted)?;
    println!("✅ Entry added: {}", name);
    Ok(())
}

pub fn get(name: &str) -> Result<()> {
    let password = prompt_password("Master password: ")?;
    let data = Self::load(&password)?;
    if let Some(entry) = data.entries.get(name) {
        println!("🔐 Username: {}", entry.username);
        println!("🔑 Password: {}", entry.password);
    } else {
        println!("❌ Entry not found");
    }
    Ok(())
}

fn load(password: &str) -> Result<VaultData> {
    let bytes = fs::read(Self::vault_path())?;
    let data: VaultData = crypto::decrypt(&bytes, password)?;
    Ok(data)
}
}