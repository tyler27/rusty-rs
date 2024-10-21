use common::document::line_map::LineMap;

#[test]
fn test_line_map_creation() {
    let text: Vec<char> = "Hello\nWorld\nThis is a test.".chars().collect();
    let line_map = LineMap::new(&text);

    assert_eq!(line_map.table.len(), 3); // Expecting 3 lines
    assert_eq!(line_map.get_line_offset(1), 0); // Line 1 starts at index 0
    assert_eq!(line_map.get_line_offset(2), 6); // Line 2 starts at index 6
    assert_eq!(line_map.get_line_offset(3), 12); // Line 3 starts at index 12
}

#[test]
#[should_panic(expected = "Line number out of range: 4 expected [1-3]")]
fn test_get_line_offset_out_of_range() {
    let text: Vec<char> = "Hello\nWorld\nThis is a test.".chars().collect();
    let line_map = LineMap::new(&text);

    // This should panic as there are only 3 lines
    line_map.get_line_offset(4);
}

#[test]
fn test_get_line_number() {
    let text: Vec<char> = "Hello\nWorld\nThis is a test.".chars().collect();
    let line_map = LineMap::new(&text);

    assert_eq!(line_map.get_line_number(0), 1); // Offset 0 is in line 1
    assert_eq!(line_map.get_line_number(5), 1); // Offset 5 is in line 1
    assert_eq!(line_map.get_line_number(6), 2); // Offset 6 is in line 2
    assert_eq!(line_map.get_line_number(12), 3); // Offset 12 is in line 3
    assert_eq!(line_map.get_line_number(25), 3); // Offset 25 is in line 3 (end of text)
}

#[test]
fn test_get_line_number_out_of_range() {
    let text: Vec<char> = "Hello\nWorld\nThis is a test.".chars().collect();
    let line_map = LineMap::new(&text);

    assert_eq!(line_map.get_line_number(30), 3); // Offset greater than the last line
}
