use clap::{arg, Command};

pub fn build_cli() -> Command {
    Command::new("dplock")
        .about("Minimal password manager - offline and secure")
        .subcommand(Command::new("init").about("Initialize a new vault"))
        .subcommand(
            Command::new("add")
                .about("Add a new password entry")
                .arg(arg!(<name> "Entry name"))
                .arg(arg!(<username> "Username")),
        )
        .subcommand(
            Command::new("get")
                .about("Get a password by name")
                .arg(arg!(<name> "Entry name"))
                .arg(arg!(--show "Print password instead of copying")),
        )
        .subcommand(
            Command::new("list")
                .about("List all saved entries")
                .arg(arg!(--filter <KEY> "Filter entries by name or username"))
                .arg(arg!(--sort <FIELD> "Sort by 'name' or 'username'")),
        )
}
