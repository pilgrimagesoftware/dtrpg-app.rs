## Context

`SettingsController` manages auth state via `AuthState::LoggedIn { email, avatar_bytes }` and exposes it to the UI through `AuthStateSnapshot`. The `set_logged_in(email, cx)` method is called from two paths: `startup_auth` (which holds the raw API key) and `sign_in` (which also holds the key). The hint only needs to be computed once, at sign-in time; it does not change until the next sign-out/sign-in cycle.

## Goals / Non-Goals

**Goals:**
- Show a masked API key hint in the authenticated Account section.
- Keep hint computation centralized (not scattered across call sites).
- Pass the hint through the existing snapshot pathway with no extra reads.

**Non-Goals:**
- Displaying the full key.
- Storing the hint persistently (computed each session from the key in memory).
- Showing the hint in the toolbar avatar button or anywhere else.

## Decisions

**Add `api_key: Option<String>` parameter to `set_logged_in`**

`set_logged_in` is the single point where "we are now logged in" is established. Threading the key through here keeps the masking logic in one place. Alternatives considered:

- _Separate `set_api_key_hint` call from each call site_: More plumbing with no benefit; callers already have the key at the same moment they call `set_logged_in`.
- _Read the keyring again at snapshot time_: Adds latency and a potential keyring error path inside a hot render loop.

**Store the masked hint string in `SettingsController` and carry it in `AuthStateSnapshot`**

`AuthStateSnapshot` already flows from controller to view. Adding `api_key_hint: Option<String>` there follows the existing pattern for `email` and `display_initial`.

**Mask format: `<first-4>••••••••<last-1>`**

- Reveals just enough for the user to recognize the key (common convention: first/last chars).
- Fixed-width bullet run (`••••••••`, 8 bullets) prevents the hint from leaking key length.
- For keys with 5 or fewer characters, show only bullets to avoid degenerate reveals.

## Risks / Trade-offs

- [Key length information] The presence or absence of revealed chars tells the user whether the key is longer than 5 chars. Accepted: this is low-sensitivity metadata for a user who already holds the key.
- [Masking computed in controller, not in view] Slight mixing of presentation concern into the controller. Accepted: the pattern is already established by `display_initial`.
