use common::document::line_column::LineColumn;

#[test]
fn test_line_column_clone() {
    let line_column: LineColumn = LineColumn::new(0, 0);
    assert_eq!(line_column, line_column.clone())
}

#[test]
fn test_line_column_lesser_than() {
    let line_column_less: LineColumn = LineColumn::new(0, 0);
    let line_column_greater: LineColumn = LineColumn::new(5, 0);
    assert_eq!(line_column_less.is_lesser_than(&line_column_greater), true);
    assert_eq!(
        line_column_less.is_greater_than(&line_column_greater),
        false
    )
}

#[test]
fn test_line_column_greater_than() {
    let line_column_less: LineColumn = LineColumn::new(0, 0);
    let line_column_greater: LineColumn = LineColumn::new(5, 0);
    assert_eq!(
        line_column_less.is_greater_than(&line_column_greater),
        false
    );
    assert_eq!(line_column_less.is_lesser_than(&line_column_greater), true);
}
