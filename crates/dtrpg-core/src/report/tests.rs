use std::cell::RefCell;

use super::*;

struct FakeGh {
    ready:  bool,
    result: Result<String, String>,
    calls:  RefCell<Vec<(Vec<String>, Option<String>)>>,
}

impl FakeGh {
    fn new(ready: bool, result: Result<String, String>) -> Self {
        Self { ready,
               result,
               calls: RefCell::default() }
    }
}

impl GhRunner for FakeGh {
    fn ready(&self) -> bool {
        self.ready
    }

    fn run(&self, args: &[&str], stdin: Option<&str>) -> Result<String, String> {
        self.calls
            .borrow_mut()
            .push((args.iter().map(|arg| (*arg).to_owned()).collect(), stdin.map(str::to_owned)));
        self.result.clone()
    }
}

fn report() -> Report {
    Report { title:       "[Bug] Catalog shows duplicates".into(),
             description: "Steps: open the app.".into(),
             diagnostics: "**Version:** 1.0.0".into(),
             log_tail:    None,
             screenshots: Vec::new(), }
}

#[test]
fn gh_ready_files_the_issue_with_the_body_on_stdin() {
    let gh = FakeGh::new(true, Ok("https://github.com/o/r/issues/9".into()));
    let outcome = submit(&report(), &gh, |_| true, |_| true);
    assert_eq!(outcome,
               Outcome::Filed("https://github.com/o/r/issues/9".into()));
    let calls = gh.calls.borrow();
    assert!(calls[0].0.contains(&REPORT_REPO.to_owned()));
    assert!(calls[0].1
                    .as_deref()
                    .is_some_and(|body| body.contains("Steps: open the app.")));
}

#[test]
fn gh_failure_is_reported_and_does_not_open_the_browser() {
    let gh = FakeGh::new(true, Err("HTTP 403".into()));
    let opened = RefCell::new(false);
    let outcome = submit(&report(),
                         &gh,
                         |_| {
                             *opened.borrow_mut() = true;
                             true
                         },
                         |_| true);
    assert_eq!(outcome, Outcome::FileFailed("HTTP 403".into()));
    assert!(!*opened.borrow());
}

#[test]
fn gh_not_ready_opens_the_compose_page() {
    let gh = FakeGh::new(false, Err(String::new()));
    let urls = RefCell::new(Vec::new());
    let outcome = submit(&report(),
                         &gh,
                         |url| {
                             urls.borrow_mut().push(url.to_owned());
                             true
                         },
                         |_| true);
    assert_eq!(outcome, Outcome::BrowserOpened);
    assert!(urls.borrow()[0].starts_with(&format!(
        "https://github.com/{REPORT_REPO}/issues/new?title="
    )));
    assert!(gh.calls.borrow().is_empty());
}

#[test]
fn gh_not_ready_and_no_browser_is_a_failure() {
    let gh = FakeGh::new(false, Err(String::new()));
    assert_eq!(submit(&report(), &gh, |_| false, |_| true),
               Outcome::BrowserFailed);
}

#[test]
fn screenshots_are_revealed_and_the_filed_issue_opens() {
    let gh = FakeGh::new(true, Ok("https://github.com/o/r/issues/9".into()));
    let mut with_shots = report();
    with_shots.screenshots = vec![PathBuf::from("/Users/me/shot.png")];
    let opened = RefCell::new(Vec::new());
    let revealed = RefCell::new(Vec::new());
    submit(&with_shots,
           &gh,
           |url| {
               opened.borrow_mut().push(url.to_owned());
               true
           },
           |path| {
               revealed.borrow_mut().push(path.to_path_buf());
               true
           });
    assert_eq!(*opened.borrow(), ["https://github.com/o/r/issues/9"]);
    assert_eq!(*revealed.borrow(), [PathBuf::from("/Users/me/shot.png")]);
}

#[test]
fn the_body_names_screenshots_by_file_name_only() {
    let mut with_shots = report();
    with_shots.screenshots = vec![PathBuf::from("/Users/me/private/shot.png")];
    let body = issue_body(&with_shots);
    assert!(body.contains("- `shot.png`"));
    assert!(!body.contains("/Users/me"));
}

#[test]
fn the_log_rides_in_the_gh_body_but_the_compose_url_only_names_the_file() {
    let mut with_log = report();
    with_log.log_tail = Some("ERROR boom".into());
    assert!(issue_body(&with_log).contains("ERROR boom"));
    assert!(!compose_url(&with_log).contains("boom"));
}

#[test]
fn the_compose_url_has_no_raw_space_or_newline() {
    let url = compose_url(&report());
    assert!(!url.contains(' ') && !url.contains('\n'));
}

#[test]
fn percent_encode_keeps_unreserved_and_escapes_the_rest() {
    assert_eq!(percent_encode("a-b_c.d~e"), "a-b_c.d~e");
    assert_eq!(percent_encode("a b&c"), "a%20b%26c");
}

#[test]
fn add_screenshots_filters_dedupes_and_caps() {
    let mut chosen = vec![PathBuf::from("a.png")];
    let skipped = add_screenshots(&mut chosen,
                                  ["a.png",
                                   "b.PNG",
                                   "notes.txt",
                                   "c.jpg",
                                   "d.gif",
                                   "e.jpeg",
                                   "f.png"].map(PathBuf::from));
    assert_eq!(chosen.len(), REPORT_MAX_SCREENSHOTS);
    assert_eq!(skipped, 2, "notes.txt and f.png (past the cap)");
}

#[test]
fn tail_lines_keeps_the_last_lines_and_survives_bad_utf8() {
    assert_eq!(tail_lines(b"1\n2\n3\n4\n", 2), "3\n4");
    assert!(tail_lines(&[b'o', 0xFF, b'k'], 5).starts_with('o'));
}
