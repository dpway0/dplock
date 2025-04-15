mod cli;
mod crypto;
mod vault;
mod utils;

use anyhow::Result;
use cli::build_cli;
use vault::Vault;
use std::path::PathBuf;

fn main() -> Result<()> {
    let matches = build_cli().get_matches();

    let vault_file = matches.get_one::<String>("vault").map(PathBuf::from);
    let mut vault = Vault::new(vault_file);

    if let Some((cmd, sub)) = matches.subcommand() {
        handle_subcommand(&mut vault, cmd, sub)?;
    }

    Ok(())
}

fn handle_subcommand(vault: &mut Vault, cmd: &str, sub: &clap::ArgMatches) -> Result<()> {
    match cmd {
        "init" => vault.init()?,
        "add" => handle_add(vault, sub)?,
        "get" => handle_get(vault, sub)?,
        "list" => handle_list(vault, sub)?,
        "remove" => handle_remove(vault, sub)?,
        "export" => handle_export(vault, sub)?,
        "import" => handle_import(vault, sub)?,
        "check-reminders" => vault.check_reminders()?,
        _ => {
            println!("❌ Unknown command: {}", cmd);
        }
    }
    Ok(())
}

fn handle_add(vault: &mut Vault, sub: &clap::ArgMatches) -> Result<()> {
    let name = sub.get_one::<String>("name").unwrap();
    let username = sub.get_one::<String>("username").unwrap();
    let use_time = sub.get_flag("time");
    let message = sub.get_one::<String>("message").map(String::as_str);
    vault.add(name, username, use_time, message)
}

fn handle_get(vault: &mut Vault, sub: &clap::ArgMatches) -> Result<()> {
    let name = sub.get_one::<String>("name").unwrap();
    let username = sub.get_one::<String>("username").map(String::as_str);
    let show = sub.get_flag("show");
    vault.get(name, username, show)
}

fn handle_list(vault: &mut Vault, sub: &clap::ArgMatches) -> Result<()> {
    let filter = sub.get_one::<String>("filter").map(String::as_str);
    let sort = sub.get_one::<String>("sort").map(String::as_str);
    vault.list(filter, sort)
}

fn handle_remove(vault: &mut Vault, sub: &clap::ArgMatches) -> Result<()> {
    let name = sub.get_one::<String>("name").unwrap();
    let index = sub.get_one::<String>("index")
        .map(|i| i.parse::<usize>().unwrap_or(0));
    vault.remove(name, index)
}

fn handle_export(vault: &mut Vault, sub: &clap::ArgMatches) -> Result<()> {
    let path = sub.get_one::<String>("path").unwrap();
    let plain = sub.get_flag("plain");
    vault.export(path, plain)
}

fn handle_import(vault: &mut Vault, sub: &clap::ArgMatches) -> Result<()> {
    let path = sub.get_one::<String>("path").unwrap();
    let plain = sub.get_flag("plain");
    vault.import(path, plain)
}