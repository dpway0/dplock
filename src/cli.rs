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
        .arg(arg!(-t --time "Enable expired/remind input (prompt for time)"))
        .arg(arg!(-m --message <MESSAGE> "Optional message or note for the entry"))
}

fn get_subcommand() -> Command {
    Command::new("get")
        .about("Get a password by name")
        .arg(arg!(<name> "Entry name"))
        .arg(arg!(<username> "Username").required(false))
        .arg(arg!(-S --show "Print password instead of copying"))
}

fn list_subcommand() -> Command {
    Command::new("list")
        .about("List all saved entries")
        .arg(arg!(-f --filter <KEY> "Filter entries by name or username"))
        .arg(arg!(-s --sort <FIELD> "Sort by 'name' or 'username'"))
}

fn remove_subcommand() -> Command {
    Command::new("remove")
        .about("Remove password entry by name (optional: specify --index to remove one entry)")
        .arg(arg!(<name> "Entry name to remove"))
        .arg(arg!(-i --index <INDEX> "Specify the index of the entry to remove (starts from 1)"))
}
fn export_subcommand() -> Command {
    Command::new("export")
        .about("Export vault to a JSON file (unencrypted)")
        .arg(arg!(<path> "Path to export the JSON file"))
        .arg(
            arg!(-p --plain "Export passwords as plain text (⚠️ unsafe)"),
        )
}

fn import_subcommand() -> Command {
    Command::new("import")
        .about("Import vault from a JSON file")
        .arg(arg!(<path> "Path to the JSON file to import"))
        .arg(arg!(-p --plain "Import passwords as plain text (⚠️ unsafe)"))
}

fn check_reminders_subcommand() -> Command {
    Command::new("check-reminders")
        .about("Check all entries and notify if any password is due for review (remind date reached)")
}

pub fn build_cli() -> Command {
    Command::new("dplock")
        .about("Minimal password manager — offline and secure")
        .arg(arg!(-v --vault <VAULT> "Specify the vault file path").global(true))
        .subcommand(init_subcommand())
        .subcommand(add_subcommand())
        .subcommand(get_subcommand())
        .subcommand(list_subcommand())
        .subcommand(remove_subcommand())
        .subcommand(export_subcommand())
        .subcommand(import_subcommand())
        .subcommand(check_reminders_subcommand())
}
