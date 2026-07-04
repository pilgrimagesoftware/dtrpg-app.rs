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

## Crash Reporting (Sentry)

Sentry crash/error reporting is opt-in and off by default. A plain `cargo build`/`cargo run` from
source never compiles Sentry in and never contacts Sentry, regardless of any environment
variables set locally.

Sentry is only active when both of the following are true:

- The binary was compiled with the `sentry` Cargo feature (`cargo build --features
  dtrpg-core/sentry`).
- A DSN is available from either:
  - the `DTRPG_SENTRY_DSN` environment variable at process startup, or
  - a value embedded at compile time (see `crates/dtrpg-core/build.rs`), which the release build
    workflow (`.github/workflows/build.yaml`) sets from the `SENTRY_DSN` repository secret. This
    is how official CI-built release artifacts report crashes without requiring end users to set
    any environment variable themselves.

Other supported variables (all optional, all read at runtime and overridable locally even in a
`sentry`-feature build):

| Variable                     | Purpose                              | Default                |
| ----------------------------- | ------------------------------------ | ---------------------- |
| `DTRPG_SENTRY_DSN`            | Sentry project DSN                   | none (Sentry disabled) |
| `DTRPG_SENTRY_ENVIRONMENT`    | Sentry `environment` tag             | `production`           |
| `DTRPG_SENTRY_RELEASE`        | Sentry `release` tag                 | crate version          |

On startup the app logs exactly one INFO line stating whether Sentry reporting is active, and if
not, why (feature not compiled in, or no DSN configured).

To test locally with the feature enabled:

```bash
DTRPG_SENTRY_DSN=https://<key>@<org>.ingest.sentry.io/<project> cargo run --features dtrpg-core/sentry
```

Maintainers: the `SENTRY_DSN` repository secret must be set under repository Settings > Secrets
and variables > Actions for the release build workflow to embed it. This is a plain project DSN,
not a sensitive credential (Sentry DSNs only permit event submission, not read access), but it
should still come from the project's own Sentry organization rather than being hardcoded anywhere
in this repository.
