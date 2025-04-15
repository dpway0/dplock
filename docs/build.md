# Build Guide for `dplock`

This guide shows how to cross-compile `dplock` for multiple platforms using Rust.

---

## 1️⃣ Prerequisites

- Rust stable toolchain: https://rustup.rs
- Install `cargo-deb` if you want `.deb` packaging:
```bash
cargo install cargo-deb
```

- For `musl` targets:
```bash
rustup target add x86_64-unknown-linux-musl aarch64-unknown-linux-musl
```

- For MacOS (Intel & Apple Silicon):
```bash
rustup target add x86_64-apple-darwin aarch64-apple-darwin
```

- For Windows:
```bash
rustup target add x86_64-pc-windows-gnu aarch64-pc-windows-gnu
```

---

## 2️⃣ Build Commands

### Linux (x86_64, static binary)
```bash
cargo build --release --target x86_64-unknown-linux-musl
```

### Linux (ARM64, static binary)
```bash
cargo build --release --target aarch64-unknown-linux-musl
```

### Ubuntu `.deb` Package (x86_64)
```bash
cargo deb --target x86_64-unknown-linux-musl
```

### macOS Intel (x86_64)
```bash
cargo build --release --target x86_64-apple-darwin
```

### macOS ARM (M1/M2 - Apple Silicon)
```bash
cargo build --release --target aarch64-apple-darwin
```

### Windows (x86_64)
```bash
cargo build --release --target x86_64-pc-windows-gnu
```

### Windows (ARM64)
```bash
cargo build --release --target aarch64-pc-windows-gnu
```

---

✅ **Output binaries** will be located in:
```
target/<target-triple>/release/dplock
```

💡 Tip: You can use `strip` to reduce binary size:
```bash
strip target/x86_64-unknown-linux-musl/release/dplock
```

---


