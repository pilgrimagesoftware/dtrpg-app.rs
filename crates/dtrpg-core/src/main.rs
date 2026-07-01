mod app;
mod logging;
mod services;
mod constants;

fn main() {
    let _log_guard = logging::init();
    app::run();
}
