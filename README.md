# dtrpg-app (Rust)

DriveThruRPG desktop frontend in Rust, built with [gpui](https://github.com/zed-industries/zed).

## Building

```bash
cargo build
cargo run -p dtrpg-core
```

## Platform Requirements

### macOS

No additional setup required. The app links `Security.framework` automatically via the `keyring` crate.

### Linux

The credential store requires a running Secret Service daemon (GNOME Keyring or KWallet) and the
`libsecret` development library:

```bash
# Debian / Ubuntu
sudo apt install libsecret-1-dev

# Fedora
sudo dnf install libsecret-devel
```

Install GNOME Keyring if no keyring daemon is running:

```bash
sudo apt install gnome-keyring
```

### Windows

No additional setup required. The app uses Windows Credential Manager via the `keyring` crate.

## Testing

```bash
cargo test --workspace
```
