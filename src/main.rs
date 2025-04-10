mod cli;
mod crypto;
mod vault;

use anyhow::Result;
use cli::build_cli;
use vault::Vault;

fn main() -> Result<()> {
    let matches = build_cli().get_matches();

    match matches.subcommand() {
        Some(("init", _)) => {
            Vault::init()?;
        }
        Some(("add", sub)) => {
            let name = sub.get_one::<String>("name").unwrap();
            let username = sub.get_one::<String>("username").unwrap();
            let password = sub.get_one::<String>("password").unwrap();
            Vault::add(name, username, password)?;
        }
        Some(("get", sub)) => {
            let name = sub.get_one::<String>("name").unwrap();
            Vault::get(name)?;
        }
        _ => {}
    }

    Ok(())
}