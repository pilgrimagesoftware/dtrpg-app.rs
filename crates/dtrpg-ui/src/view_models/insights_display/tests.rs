use super::*;

fn insights(titles: usize, downloaded: usize) -> Insights {
    Insights { titles,
               downloaded,
               ..Insights::default() }
}

#[test]
fn availability_slices_add_up_to_the_titles() {
    let slices = availability_slices(&insights(18, 9),
                                     LiveAvailability { downloading: 2,
                                                        failed:      1, });

    assert_eq!(slices,
               [(Availability::OnDevice, 9),
                (Availability::NotDownloaded, 6),
                (Availability::Downloading, 2),
                (Availability::Failed, 1)]);
    assert_eq!(slices.iter().map(|(_, count)| count).sum::<usize>(), 18);
}

#[test]
fn live_counts_larger_than_the_cloud_titles_are_held_to_them() {
    let slices = availability_slices(&insights(4, 3),
                                     LiveAvailability { downloading: 5,
                                                        failed:      5, });

    assert_eq!(slices.iter().map(|(_, count)| count).sum::<usize>(), 4);
}

#[test]
fn top_rows_scale_bars_to_the_longest() {
    let counts = [Count { label: "A".into(),
                          count: 4, },
                  Count { label: "B".into(),
                          count: 1, }];

    let (rows, all) = top_rows(&counts, 8);

    assert_eq!(all, 2);
    assert!((rows[1].fraction - 0.25).abs() < f32::EPSILON);
}

#[test]
fn a_month_is_labelled_with_two_digit_month_and_year() {
    assert_eq!(month_label(&MonthCount { year:  2026,
                                         month: 7,
                                         count: 1, }),
               "07/26");
}
