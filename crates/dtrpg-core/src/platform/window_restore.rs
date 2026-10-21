//! Whether windows that were open at quit reopen at the next launch.
//!
//! macOS has a system setting for this ("Close windows when quitting an
//! application", in System Settings > Desktop & Dock), and Libri follows it.
//! Other platforms have no such setting, so Libri has its own switch in
//! Settings > Native Integration.

use crate::data::native_preferences::NativePreferencesFile;

/// Whether windows that were open at quit should reopen at launch.
///
/// # Examples
///
/// ```
/// use dtrpg_core::data::native_preferences::NativePreferencesFile;
/// use dtrpg_core::platform::window_restore::reopen_windows_at_launch;
///
/// let _reopen = reopen_windows_at_launch(&NativePreferencesFile::default());
/// ```
#[must_use]
pub fn reopen_windows_at_launch(native: &NativePreferencesFile) -> bool {
    if cfg!(target_os = "macos") {
        macos_keeps_windows_on_quit()
    }
    else {
        native.reopen_windows_on_launch
    }
}

/// Reads the macOS "Close windows when quitting an application" setting.
#[cfg(target_os = "macos")]
fn macos_keeps_windows_on_quit() -> bool {
    let output =
        std::process::Command::new("defaults").args(["read", "-g", "NSQuitAlwaysKeepsWindows"])
                                              .output()
                                              .ok()
                                              .filter(|output| output.status.success())
                                              .map(|output| {
                                                  String::from_utf8_lossy(&output.stdout)
                                                               .into_owned()
                                              });
    parse_keeps_windows(output.as_deref())
}

#[cfg(not(target_os = "macos"))]
fn macos_keeps_windows_on_quit() -> bool {
    true
}

/// Interprets the output of `defaults read -g NSQuitAlwaysKeepsWindows`.
///
/// The system writes `0` when the user ticks "Close windows when quitting an
/// application". Anything else, including no value at all, keeps windows.
#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
fn parse_keeps_windows(defaults_output: Option<&str>) -> bool {
    defaults_output.map(str::trim) != Some("0")
}

#[cfg(test)]
mod tests {
    use super::parse_keeps_windows;

    #[test]
    fn windows_are_kept_unless_the_system_says_to_close_them() {
        assert!(!parse_keeps_windows(Some("0\n")));
        assert!(parse_keeps_windows(Some("1\n")));
        assert!(parse_keeps_windows(None));
    }
}
