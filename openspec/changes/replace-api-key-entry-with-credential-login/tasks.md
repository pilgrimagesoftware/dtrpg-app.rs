## 1. Settings Account tab sign-in form

- [x] 1.1 Replace the API key field in the Settings Account tab sign-in form
      with email and password fields
- [x] 1.2 Update submit-button enablement logic for the two-field form
- [x] 1.3 Pre-fill the email field when a stored entry has an email but an
      invalid or expired application key

## 2. SettingsController and login flow

- [x] 2.1 Update `SettingsController` to call the SDK credential exchange
      (`dtrpg_sdk::credential_login::login_with_credentials` or equivalent),
      then the existing `LoginService::authenticate` call, surfacing distinct
      errors for each failure mode
- [x] 2.2 Update loading/disabled state handling to span both calls

## 3. Credential storage

- [x] 3.1 Extend the stored credential payload (`Credential` /
      `KeyringCredentialStore`) to include account email alongside the
      application key
- [x] 3.2 Update `KeyringCredentialStore` read path to tolerate legacy
      entries with no email field
- [x] 3.3 Update tests in `crates/dtrpg-ui/src/credentials/store.rs` for the
      new payload shape and legacy read compatibility

## 4. Verification

- [x] 4.1 Run `cargo test --workspace`
- [x] 4.2 Run `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] 4.3 Manually verify sign-in with valid and invalid credentials against
      the real DriveThruRPG website endpoints
- [ ] 4.4 Verify a pre-existing legacy (key-only) keychain entry still allows
      silent startup reauthentication until the key is rejected
