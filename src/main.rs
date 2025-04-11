mod cli;
mod crypto;
mod vault;
mod utils;

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
            Vault::add(name, username)?;
        }
        Some(("get", sub)) => {
            let name = sub.get_one::<String>("name").unwrap();
            let show = sub.get_flag("show");
            Vault::get(name, show)?;
        }
        Some(("list", sub)) => {
            let filter = sub.get_one::<String>("filter").map(|s| s.as_str());
            let sort = sub.get_one::<String>("sort").map(|s| s.as_str());
            Vault::list(filter, sort)?;
        }
        _ => {}
    }

    Ok(())
}