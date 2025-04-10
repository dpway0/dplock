# 🔐 dplock

**dplock** is a minimal, offline-first, and secure password manager built with Rust. It uses strong encryption and provides a simple command-line interface (CLI) for managing your credentials.

## ✨ Features

- Fully encrypted vault using [ChaCha20Poly1305](https://docs.rs/chacha20poly1305/)
- Master password secured via [Argon2](https://docs.rs/argon2/)
- Zero network access – works entirely **offline**
- Simple CLI interface:
  - `init` – initialize a new vault
  - `add` – add a new entry
  - `get` – retrieve an entry

## 🧪 Quick Demo

```bash
$ dplock init
🔐 Vault initialized!

$ dplock add github dpway secret123
✅ Entry added: github

$ dplock get github
🔐 Username: dpway
🔑 Password: secret123
```

## 🛠 Installation

**Requirements:**

- Rust (v1.86 or newer)
- Linux / macOS / Windows

**Build from source:**

```bash
git clone https://github.com/dpway0/dplock
cd dplock
cargo build --release
```

The compiled binary will be located at `target/release/dplock`.

## 🔐 Vault & Security

- Vault is stored at: `~/.dplock/vault.bin`
- Data is encrypted using a key derived from your master password
- No telemetry, no cloud, no syncing – your data stays local

## 🤝 Contributing

Contributions are welcome! Feel free to open issues, suggest features, or send pull requests.

---

Made with ❤️ in Rust.
