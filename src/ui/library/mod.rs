//! Library UI feature modules.

pub mod controller {
    pub mod library_controller;
}

pub mod model {
    pub mod library_data;
}

pub mod views {
    pub mod controls_view;
    pub mod detail_pane_view;
    pub mod library_pane_view;
    pub mod root_view;
}

pub use views::root_view::launch;
