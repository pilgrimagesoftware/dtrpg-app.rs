use super::*;

fn share(label: &str, mb: f64) -> Share {
    Share { label: label.to_owned(),
            mb }
}

#[test]
fn the_areas_are_in_proportion_to_the_values_and_fill_the_box() {
    let values = [412.6, 381.1, 355.7, 312.7, 268.0, 245.9, 221.0, 204.3, 96.4];
    let total: f64 = values.iter().sum();

    let rects = layout(&values, 3.5);

    let filled: f64 = rects.iter().map(Rect::area).sum();
    assert!((filled - 1.0).abs() < 1e-9, "the tiles fill the box");
    for (rect, value) in rects.iter().zip(values) {
        assert!((rect.area() - value / total).abs() < 1e-9);
    }
}

#[test]
fn the_tiles_stay_inside_the_box() {
    let rects = layout(&[50.0, 30.0, 10.0, 5.0, 3.0, 2.0], 4.0);

    for rect in rects {
        assert!(rect.x >= -1e-9 && rect.y >= -1e-9);
        assert!(rect.x + rect.w <= 1.0 + 1e-9);
        assert!(rect.y + rect.h <= 1.0 + 1e-9);
    }
}

#[test]
fn a_value_that_is_not_above_zero_gets_no_room() {
    let rects = layout(&[10.0, 0.0, -3.0, 10.0], 2.0);

    assert_eq!(rects[1], Rect::EMPTY);
    assert_eq!(rects[2], Rect::EMPTY);
    assert!((rects[0].area() + rects[3].area() - 1.0).abs() < 1e-9);
}

#[test]
fn nothing_to_lay_out_gives_empty_rectangles() {
    assert!(layout(&[], 2.0).is_empty());
    assert_eq!(layout(&[0.0], 2.0), [Rect::EMPTY]);
}

#[test]
fn the_tail_is_gathered_into_one_other_tile() {
    let shares = [share("A", 50.0),
                  share("B", 30.0),
                  share("C", 15.0),
                  share("D", 5.0)];

    let tiles = tiles(&shares, 2, "Other", 3.0);

    assert_eq!(tiles.len(), 3);
    assert_eq!(tiles[2].label, "Other");
    assert!((tiles[2].mb - 20.0).abs() < 1e-9);
}

#[test]
fn no_other_tile_is_added_when_every_publisher_has_its_own() {
    let tiles = tiles(&[share("A", 5.0), share("B", 4.0)], 8, "Other", 3.0);

    assert_eq!(tiles.len(), 2);
}
