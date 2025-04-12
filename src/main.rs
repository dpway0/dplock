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
        "add" => handle_add_command(sub)?,
        "get" => handle_get_command(sub)?,
        "list" => handle_list_command(sub)?,
        _ => {}
    }
    Ok(())
}

fn handle_add_command(sub: &clap::ArgMatches) -> Result<()> {
    let name = sub.get_one::<String>("name").unwrap();
    let username = sub.get_one::<String>("username").unwrap();
    Vault::add(name, username)
}

fn handle_get_command(sub: &clap::ArgMatches) -> Result<()> {
    let name = sub.get_one::<String>("name").unwrap();
    let show = sub.get_flag("show");
    Vault::get(name, show)
}

fn handle_list_command(sub: &clap::ArgMatches) -> Result<()> {
    let filter = sub.get_one::<String>("filter").map(|s| s.as_str());
    let sort = sub.get_one::<String>("sort").map(|s| s.as_str());
    Vault::list(filter, sort)
}