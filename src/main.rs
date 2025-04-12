mod cli;
mod crypto;
mod vault;
mod utils;

use anyhow::Result;
use cli::build_cli;
use vault::Vault;

fn main() -> Result<()> {
    let matches = build_cli().get_matches();

    if let Some((cmd, sub)) = matches.subcommand() {
        handle_subcommand(cmd, sub)?;
    }

    Ok(())
}

fn handle_subcommand(cmd: &str, sub: &clap::ArgMatches) -> Result<()> {
    match cmd {
        "init" => Vault::init()?,
        "add" => handle_add(sub)?,
        "get" => handle_get(sub)?,
        "list" => handle_list(sub)?,
        "remove" => handle_remove(sub)?,
        "export" => handle_export(sub)?,
        "import" => handle_import(sub)?,
        _ => {
            println!("❌ Unknown command: {}", cmd);
        }
    }
    Ok(())
}

fn handle_add(sub: &clap::ArgMatches) -> Result<()> {
    let name = sub.get_one::<String>("name").unwrap();
    let username = sub.get_one::<String>("username").unwrap();
    Vault::add(name, username)
}

fn handle_get(sub: &clap::ArgMatches) -> Result<()> {
    let name = sub.get_one::<String>("name").unwrap();
    let show = sub.get_flag("show");
    Vault::get(name, show)
}

fn handle_list(sub: &clap::ArgMatches) -> Result<()> {
    let filter = sub.get_one::<String>("filter").map(String::as_str);
    let sort = sub.get_one::<String>("sort").map(String::as_str);
    Vault::list(filter, sort)
}

fn handle_remove(sub: &clap::ArgMatches) -> Result<()> {
    let name = sub.get_one::<String>("name").unwrap();
    let index = sub.get_one::<String>("index")
        .map(|i| i.parse::<usize>().unwrap_or(0));
    Vault::remove(name, index)
}

fn handle_export(sub: &clap::ArgMatches) -> Result<()> {
    let path = sub.get_one::<String>("path").unwrap();
    let plain = sub.get_flag("plain");
    Vault::export(path, plain)
}

fn handle_import(sub: &clap::ArgMatches) -> Result<()> {
    let path = sub.get_one::<String>("path").unwrap();
    let plain = sub.get_flag("plain");
    Vault::import(path, plain)
}