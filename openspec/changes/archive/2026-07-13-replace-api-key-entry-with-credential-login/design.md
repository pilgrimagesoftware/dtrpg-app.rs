## Context

The login/sign-in flow lives in `SettingsController`
(`crates/dtrpg-ui/src/controllers/settings.rs`), which holds `api_key_draft`,
`api_key_input`, `sign_in_in_progress`, and `sign_in_error`, and calls
`LoginService::authenticate(api_key)` (implemented by `SdkLoginService` in
`crates/dtrpg-core/src/services/login.rs`) on submit. Credentials are
persisted via `KeyringCredentialStore` (`crates/dtrpg-ui/src/credentials`),
which stores a single `Credential { service, account, secret }` under the
`api-key` account key.

`dtrpg-sdk/rust` will add `login_with_credentials(email, password, config) ->
Result<String, ClientError>` (see
`dtrpg-sdk/rust/openspec/changes/credential-login-exchange`), which this
change consumes.

## Goals / Non-Goals

**Goals:**
- Replace the single API key field with email and password fields, keeping
  existing disabled/loading/error states.
- Call the SDK credential exchange, then the existing `authenticate` call,
  from `SettingsController`, surfacing distinct errors for each step.
- Persist the account email alongside the application key so the login view
  can pre-fill it on reauthentication.

**Non-Goals:**
- Do not change `LoginService::authenticate`'s signature or
  `SdkLoginService`'s use of `auth_client::authenticate` — it continues to
  consume an application key exactly as it does today.
- Do not migrate existing key-only keychain entries; they remain valid for
  silent reauthentication until the key is rejected.

## Decisions

- **Two-field draft state on `SettingsController`**: replace `api_key_draft`
  with `email_draft` and `password_draft` (names TBD at implementation time),
  and replace `api_key_input` with two `Entity<InputState>` fields. Keep
  `sign_in_in_progress` and `sign_in_error` spanning both the credential
  exchange and the token exchange.
- **Target `unauthenticated-main-window`, not a new `auth-login-view`
  capability**: this repo removed its standalone login window; the
  umbrella/top-level `auth-login-view` capability describes a UI surface that
  no longer exists here. The Settings Account tab sign-in form is already
  covered by `unauthenticated-main-window`'s "Sign-in MUST be available from
  the Settings Account tab" requirement, so this change modifies that
  requirement instead of introducing a stale capability name.
- **Sequential calls in `startup_auth`/sign-in path**: call
  `login_with_credentials` first; only call `LoginService::authenticate` with
  the returned application key if the first call succeeds. On failure at
  either step, set `sign_in_error` to a message identifying which step
  failed.
- **Credential payload gains `email`, not a new store type**: extend
  `Credential` with an `email: Option<String>` field (or store a JSON-encoded
  `{email, key}` payload as the keychain secret — decided at implementation
  time), kept under the existing `api-key` account key. `KeyringCredentialStore::load`
  tolerates entries with no email (legacy key-only entries) by returning
  `email: None`.
- **Pre-fill on reauthentication**: when a stored entry has an email but the
  application key is rejected by `authenticate`, the login view pre-fills the
  email field so the user only re-enters their password.

## Risks / Trade-offs

- Existing users have a keychain entry containing only an application key,
  with no email → treated as still valid for silent reauthentication; only a
  fresh email/password login is required when the entry is entirely absent or
  the application key is rejected.
- Two sequential website calls plus the existing `auth_key` call means three
  network round-trips to complete one login → acceptable for an interactive
  login action.

## Migration Plan

1. Update `SettingsController` and the login view for the two-field form.
2. Wire the SDK credential exchange into the sign-in path, before the
   existing `authenticate` call.
3. Extend `Credential`/`KeyringCredentialStore` for the email field, with
   legacy read compatibility.
4. No data migration script needed — legacy entries keep working until the
   stored key is next rejected.
