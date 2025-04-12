use clap::{arg, Command};

fn init_subcommand() -> Command {
    Command::new("init")
        .about("Initialize a new vault")
}

fn add_subcommand() -> Command {
    Command::new("add")
        .about("Add a new password entry")
        .arg(arg!(<name> "Entry name"))
        .arg(arg!(<username> "Username"))
}

fn get_subcommand() -> Command {
    Command::new("get")
        .about("Get a password by name")
        .arg(arg!(<name> "Entry name"))
        .arg(arg!(--show "Print password instead of copying"))
}

fn list_subcommand() -> Command {
    Command::new("list")
        .about("List all saved entries")
        .arg(arg!(--filter <KEY> "Filter entries by name or username"))
        .arg(arg!(--sort <FIELD> "Sort by 'name' or 'username'"))
}

fn remove_subcommand() -> Command {
    Command::new("remove")
        .about("Remove password entry by name (optional: specify --index to remove one entry)")
        .arg(arg!(<name> "Entry name to remove"))
        .arg(arg!(--index <INDEX> "Specify the index of the entry to remove (starts from 1)"))
}

pub fn build_cli() -> Command {
    Command::new("dplock")
        .about("Minimal password manager — offline and secure")
        .subcommand(init_subcommand())
        .subcommand(add_subcommand())
        .subcommand(get_subcommand())
        .subcommand(list_subcommand())
        .subcommand(remove_subcommand())
}
