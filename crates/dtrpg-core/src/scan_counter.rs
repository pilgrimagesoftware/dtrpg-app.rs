//! Counts reads that walk the whole catalog, so a test can prove the render
//! path never performs one.
//!
//! GPUI re-renders on every keystroke, so any whole-catalog read that lands
//! on the render path runs per key. Every fix that removed one has later been
//! re-added by a feature branch merged over it, because nothing enforced the
//! rule. The render guard in `dtrpg_ui`'s controllers closes that gap: reset,
//! read one frame's worth of view state, and expect the count to still be
//! zero. The recorded [`std::panic::Location`] names the call site that
//! scanned, so a failure points straight at the offender.
//!
//! The counters are per-thread: rendering happens on the main thread, tests
//! run concurrently on their own threads, and a worker thread's legitimate
//! event-path scan must not trip another test's guard.
//!
//! The counter is compiled into production too. A cell write per index query
//! is noise beside the query itself.

use std::{
    cell::{Cell, RefCell},
    panic::Location,
};

thread_local! {
    /// Whole-catalog reads on this thread since the last [`reset_catalog_scans`].
    static SCANS: Cell<u64> = const { Cell::new(0) };

    /// Reads that materialize every catalog item into memory, since the last
    /// reset. Cheaper aggregate SQL over the whole table counts as a scan but
    /// not a materialization; event paths may scan, but materializing on a
    /// keystroke is the sluggishness regression.
    static MATERIALIZATIONS: Cell<u64> = const { Cell::new(0) };

    /// The call site of the thread's most recent scan, for failure messages.
    static LAST_SCAN_CALLER: RefCell<Option<&'static Location<'static>>> =
        const { RefCell::new(None) };
}

/// Records a scan whose outermost caller is `caller`. Annotated index reads
/// call this through [`catalog_scan`]; nothing else should.
pub fn note_catalog_scan(caller: &'static Location<'static>) {
    SCANS.with(|scans| scans.set(scans.get() + 1));
    LAST_SCAN_CALLER.with(|last| *last.borrow_mut() = Some(caller));
}

/// Records a read that materializes every catalog item into memory. Call it
/// at the top of each full-table item read, alongside [`catalog_scan`].
pub fn note_catalog_materialization(caller: &'static Location<'static>) {
    SCANS.with(|scans| scans.set(scans.get() + 1));
    MATERIALIZATIONS.with(|m| m.set(m.get() + 1));
    LAST_SCAN_CALLER.with(|last| *last.borrow_mut() = Some(caller));
}

/// Reads that materialized the whole catalog on this thread since the reset.
#[must_use]
pub fn catalog_materialization_count() -> u64 {
    MATERIALIZATIONS.with(Cell::get)
}

/// The call site of the most recent whole-catalog materialization, if any.
#[must_use]
pub fn last_catalog_materializer() -> Option<&'static Location<'static>> {
    LAST_SCAN_CALLER.with(|last| *last.borrow())
}

/// Whole-catalog scans counted on this thread since the last reset.
#[must_use]
pub fn catalog_scan_count() -> u64 {
    SCANS.with(Cell::get)
}

/// The call site of the most recent scan on this thread, if any.
#[must_use]
pub fn last_catalog_scanner() -> Option<&'static Location<'static>> {
    LAST_SCAN_CALLER.with(|last| *last.borrow())
}

/// Forgets past scans on this thread. Tests call this before the pass they
/// want policed.
pub fn reset_catalog_scans() {
    SCANS.with(|scans| scans.set(0));
    MATERIALIZATIONS.with(|m| m.set(0));
    LAST_SCAN_CALLER.with(|last| *last.borrow_mut() = None);
}

/// Instruments a whole-catalog read. Call it at the top of every index query
/// that reads the catalog in full, with `#[track_caller]` on the host
/// function; wrapper functions the read passes through are `#[track_caller]`
/// too, so the recorded location resolves to the scan's real caller.
#[macro_export]
macro_rules! catalog_scan {
    () => {
        $crate::scan_counter::note_catalog_scan(::std::panic::Location::caller())
    };
}

/// Instruments a read that materializes every catalog item into memory.
#[macro_export]
macro_rules! catalog_materialization {
    () => {
        $crate::scan_counter::note_catalog_materialization(::std::panic::Location::caller())
    };
}

#[cfg(test)]
// Test joins need the thread's payload; a poisoned join here is a test failure
// either way.
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn a_scan_is_counted_until_the_next_reset() {
        reset_catalog_scans();
        note_catalog_scan(Location::caller());
        assert_eq!(catalog_scan_count(), 1);
        assert!(last_catalog_scanner().is_some());

        reset_catalog_scans();
        assert_eq!(catalog_scan_count(), 0);
        assert!(last_catalog_scanner().is_none());
    }

    #[test]
    fn counts_are_per_thread() {
        reset_catalog_scans();
        note_catalog_scan(Location::caller());
        let other = std::thread::spawn(|| {
                        reset_catalog_scans();
                        catalog_scan_count()
                    }).join()
                      .unwrap();
        assert_eq!(other, 0, "a fresh thread learns nothing from this one");
        assert_eq!(catalog_scan_count(), 1);
    }
}
