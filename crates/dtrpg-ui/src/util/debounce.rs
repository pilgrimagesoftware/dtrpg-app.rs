//! Running something once after a burst of requests has gone quiet.
//!
//! Resizing a window or dragging a panel divider reports a new size on every
//! frame. Saving each one means a file read and write per frame, which makes
//! the drag stutter, so the save waits until the drag stops.

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use gpui::BackgroundExecutor;

/// How long a burst has to be quiet before its last request is carried out.
pub const SAVE_DELAY: Duration = Duration::from_millis(400);

/// Carries out only the last of a run of requests, once `delay` has passed
/// with no newer one. The action runs on a background thread.
#[derive(Clone, Default)]
pub struct Debounce {
    /// Counts the requests, so a waiting one can tell a newer one came.
    latest: Arc<AtomicU64>,
}

impl Debounce {
    /// A debounce with nothing waiting.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Asks for `action` to run after `delay`, unless another request comes
    /// first, in which case that one replaces it.
    pub fn run(&self, executor: &BackgroundExecutor, delay: Duration,
               action: impl FnOnce() + Send + 'static) {
        let mine = self.latest.fetch_add(1, Ordering::SeqCst) + 1;
        let latest = Arc::clone(&self.latest);
        let timer = executor.timer(delay);
        executor.spawn(async move {
                    timer.await;
                    if latest.load(Ordering::SeqCst) == mine {
                        action();
                    }
                })
                .detach();
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::AtomicUsize;

    use gpui::TestAppContext;

    use super::*;

    #[gpui::test]
    fn only_the_last_request_of_a_burst_runs(cx: &mut TestAppContext) {
        let debounce = Debounce::new();
        let runs = Arc::new(AtomicUsize::new(0));
        let executor = cx.executor();

        for _ in 0..5 {
            let runs = Arc::clone(&runs);
            debounce.run(&executor, SAVE_DELAY, move || {
                        runs.fetch_add(1, Ordering::SeqCst);
                    });
        }
        cx.executor().advance_clock(SAVE_DELAY * 2);
        cx.run_until_parked();

        assert_eq!(runs.load(Ordering::SeqCst), 1);
    }

    #[gpui::test]
    fn nothing_runs_before_the_delay_has_passed(cx: &mut TestAppContext) {
        let debounce = Debounce::new();
        let runs = Arc::new(AtomicUsize::new(0));
        let executor = cx.executor();
        let counted = Arc::clone(&runs);

        debounce.run(&executor, SAVE_DELAY, move || {
                    counted.fetch_add(1, Ordering::SeqCst);
                });
        cx.run_until_parked();

        assert_eq!(runs.load(Ordering::SeqCst), 0);
    }
}
