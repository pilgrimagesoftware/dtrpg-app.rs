## Why

This app removed the standalone login window (`always-open-main-window`); the
Settings Account tab now asks the user to paste a raw DriveThruRPG API key,
with no page on the DriveThruRPG website that shows a user their key. Once
`dtrpg-sdk/rust` exposes `login_with_credentials(email, password)`, the app
can let the user sign in the way they already do on the website: email and
password.

Note: the umbrella proposal describes this as a "login window" change. In
this repo, sign-in already lives in the Settings Account tab
(`unauthenticated-main-window`'s "Sign-in MUST be available from the Settings
Account tab" requirement), not a separate window. This child change targets
that requirement instead of a nonexistent login window, and flags the
discrepancy back to the umbrella proposal for correction.

This is a child change of the umbrella proposal
`dtrpg/openspec/changes/replace-api-key-entry-with-credential-login`, and
depends on the SDK child change
`dtrpg-sdk/rust/openspec/changes/credential-login-exchange`.

## What Changes

- Replace the API key text field in the Settings Account tab sign-in form
  with email and password fields.
- Update `SettingsController` (and `SdkLoginService`) to call the SDK
  credential exchange first, then the existing `authenticate` call,
  surfacing distinct errors for each failure mode.
- Extend the stored credential to include the account email alongside the
  application key, and pre-fill the email field on reauthentication.
- **BREAKING**: an existing keychain entry with only an application key (no
  email) is still valid for silent reauthentication until the key is
  rejected; once rejected, the user must sign in again with email/password.

## Capabilities

### Modified Capabilities
- `unauthenticated-main-window`: the Settings Account tab sign-in form
  collects email and password instead of a raw API key, and the submission
  flow performs the credential exchange before the existing token exchange.

### New Capabilities
- `credential-store`: this repo has no existing `credential-store` capability
  spec of its own (credential persistence behavior was previously
  undocumented at the spec level); this change adds one, scoped to
  `dtrpg-app/rust`, covering the email-alongside-key payload and legacy
  read compatibility.

## Impact

- `dtrpg-app/rust/crates/dtrpg-core/src/services/login.rs`: calls the new SDK
  credential exchange before the existing `authenticate` call.
- `dtrpg-app/rust/crates/dtrpg-ui/src/credentials`: `Credential`/`store.rs`
  extended to carry email; `KeyringCredentialStore` read path stays
  compatible with legacy key-only entries.
- `dtrpg-app/rust/crates/dtrpg-ui/src/controllers/settings.rs` (`SettingsController`)
  and the Settings Account tab view: two fields instead of one, updated
  enablement/loading/error handling.
- Upstream/umbrella dependency: `dtrpg/openspec/changes/replace-api-key-entry-with-credential-login`.
- SDK dependency: `dtrpg-sdk/rust/openspec/changes/credential-login-exchange`.
