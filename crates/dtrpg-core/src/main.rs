mod app;
mod constants;
mod logging;
mod services;

fn main() {
    let _log_guard = logging::init();
    app::run();
}
