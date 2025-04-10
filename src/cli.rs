use clap::{arg, Command};

pub fn build_cli() -> Command {
    Command::new("dplock")
        .about("Minimal password manager - offline and secure")
        .subcommand(Command::new("init").about("Initialize a new vault"))
        .subcommand(
            Command::new("add")
                .about("Add a new password entry")
                .arg(arg!(<name> "Entry name"))
                .arg(arg!(<username> "Username"))
                .arg(arg!(<password> "Password")),
        )
        .subcommand(
            Command::new("get")
                .about("Get a password by name")
                .arg(arg!(<name> "Entry name")),
        )
}
