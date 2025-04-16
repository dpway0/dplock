# 🔐 dplock

**dplock** is a minimal, offline-first, and secure password manager built with Rust. It uses strong encryption and provides a simple command-line interface (CLI) for managing your credentials.

## ✨ Features

- **Strong Encryption**: Fully encrypted vault using [ChaCha20Poly1305](https://docs.rs/chacha20poly1305/).
- **Secure Master Password**: Secured via [Argon2](https://docs.rs/argon2/), a memory-hard password hashing algorithm.
- **Offline-First**: Zero network access – works entirely **offline**.
- **Simple CLI Commands**:
  - `init` – Initialize a new vault.
  - `add` – Add a new entry with a name and username.
  - `get` – Retrieve an entry by name (optionally print the password with `--show`).
  - `list` – List all saved entries with optional filters and sorting.
  - `remove` – Remove a password entry by name (optionally specify an index).
  - `export` – Export the vault to a JSON file for backup or migration purposes.
  - `import` – Import a vault from a JSON file.
  - `check-reminders` – Check and notify if any password is due for review (remind date reached).
  - **Utility**: Clear the terminal screen for better readability.
- **Check Reminders**: The `check_reminders` feature allows users to review entries with upcoming or overdue reminders. This feature is currently under development and will be available soon.

## 🧪 Quick Demo

```bash
$ dplock init
🔐 Vault initialized!

$ dplock add github dpway
🔑 Enter password: ******
✅ Entry added: github

$ dplock get github
🔐 Username: dpway
🔑 Password copied to clipboard!

$ dplock get github --show
🔐 Username: dpway
🔑 Password: secret123

$ dplock list --filter git --sort name
📒 Entries:
• github (👤 dpway)
```

## 🛠 Installation

### Requirements

- **Rust**: Version 1.86 or newer.
- **Supported Platforms**: Linux, macOS

### Prebuilt binaries

🏁 Ubuntu

Download the .deb package for easy installation:

Download [dplock_0.3.0-1_amd64.deb](https://github.com/dpway0/dplock/releases/latest/download/dplock_0.3.0-1_amd64.deb) and install:
```
sudo dpkg -i dplock_0.2.0-1_amd64.deb
```

🍏 macOS

Download the binary from [Releases](https://github.com/dpway0/dplock/releases), then:

```
# For Intel-based Macs
chmod +x dplock-0.3.0-x86_64-macos 
mv dplock-0.3.0-x86_64-macos /usr/local/bin/dplock

# For Apple Silicon (M1/M2/M3)
chmod +x dplock-0.3.0-aarch64-macos
sudo mv dplock-0.3.0-aarch64-macos /usr/local/bin/dplock

```

### Build from Source

```bash
git clone https://github.com/dpway0/dplock
cd dplock
cargo build --release
```

The compiled binary will be located at `target/release/dplock`.

## 🔐 Vault & Security

- **Vault Location**: `~/.dplock/vault.bin`.
- **Encryption**: Data is encrypted using a key derived from your master password.
- **Privacy**: No telemetry, no cloud, no syncing – your data stays local.

## 📖 CLI Commands

### `init`
Initialize a new vault.

```bash
dplock init
```

### `add`
Add a new password entry.

```bash
dplock add <name> <username> [--time]
```

- `<name>`: The name of the entry (e.g., "github").
- `<username>`: The username associated with the entry.
- `--time`: Enable expired/remind input (prompt for time).

### `get`
Retrieve a password by name.

```bash
dplock get <name> [<username>] [--show]
```

- `<name>`: The name of the entry to retrieve.
- `<username>` (optional): The username associated with the entry.
- `--show`: Print the password instead of copying it to the clipboard.

### `list`
List all saved entries.

```bash
dplock list [--filter <KEY>] [--sort <FIELD>]
```

- `--filter <KEY>`: Filter entries by name or username.
- `--sort <FIELD>`: Sort entries by `name` or `username`.

### `remove`
Remove a password entry by name.

```bash
dplock remove <name> [--index <INDEX>]
```

- `<name>`: The name of the entry to remove.
- `--index <INDEX>`: Specify the index of the entry to remove (starts from 1).

### `export`
Export the vault to a JSON file (unencrypted).

```bash
dplock export <path> [--plain]
```

- `<path>`: The path to export the JSON file.
- `--plain`: Export passwords as plain text (⚠️ unsafe).

### `import`
Import a vault from a JSON file.

```bash
dplock import <path> [--plain]
```

- `<path>`: The path to the JSON file to import.
- `--plain`: Import passwords as plain text (⚠️ unsafe).

### `check-reminders`
Check all entries and notify if any password is due for review (remind date reached).

```bash
dplock check-reminders
```

## 🤝 Contributing

Contributions are welcome! Feel free to:

- Open issues for bugs or feature requests.
- Suggest improvements or new features.
- Submit pull requests.

### Development Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/dpway0/dplock
   cd dplock
   ```
2. Build and test:
   ```bash
   cargo build
   cargo test
   ```

## 💖 Support This Project

If you like `dplock` and want to support development, you can donate via PayPal:

👉 [Donate via PayPal](https://paypal.me/zifuong)

---

Made with ❤️ in Rust.