//! App-level and window-level gpui actions for Libri.

use gpui::actions;

// App-level actions
actions!(
    libri,
    [
        Quit,
        HideApplication,
        HideOthers,
        ShowAll,
        ShowSettings,
        About,
    ]
);

// Window-level actions
actions!(libri, [Minimize, Zoom, ToggleFullscreen]);

// Thin no-op action types for OS text-editing routing via OsAction
actions!(libri, [Undo, Redo, Cut, Copy, Paste, SelectAll]);
