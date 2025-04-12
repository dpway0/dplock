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
  - **Utility**: Clear the terminal screen for better readability.

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
- **Supported Platforms**: Linux, macOS, Windows.

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
dplock add <name> <username>
```

- `<name>`: The name of the entry (e.g., "github").
- `<username>`: The username associated with the entry.

### `get`
Retrieve a password by name.

```bash
dplock get <name> [--show]
```

- `<name>`: The name of the entry to retrieve.
- `--show`: Print the password instead of copying it to the clipboard.

### `list`
List all saved entries.

```bash
dplock list [--filter <KEY>] [--sort <FIELD>]
```

- `--filter <KEY>`: Filter entries by name or username.
- `--sort <FIELD>`: Sort entries by `name` or `username`.

### `clear`
Clear the terminal screen.

```bash
dplock clear
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

---

Made with ❤️ in Rust.
