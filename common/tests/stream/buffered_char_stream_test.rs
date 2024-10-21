use common::stream::buffered_char_stream::BufferedCharStream;
// Import the items from the parent module
use std::io::Cursor;

#[test]
fn test_take_and_peek() {
    let input = "Hello, world!";
    let mut stream = BufferedCharStream::new(Cursor::new(input)).unwrap();

    assert_eq!(stream.peek(), Some('H'));
    assert_eq!(stream.take(), Some('H'));
    assert_eq!(stream.peek(), Some('e'));
    assert_eq!(stream.take(), Some('e'));
    assert_eq!(stream.take(), Some('l'));
    assert_eq!(stream.take(), Some('l'));
    assert_eq!(stream.take(), Some('o'));
    assert_eq!(stream.take(), Some(','));
    assert_eq!(stream.take(), Some(' '));
    assert_eq!(stream.take(), Some('w'));
    assert_eq!(stream.take(), Some('o'));
    assert_eq!(stream.take(), Some('r'));
    assert_eq!(stream.take(), Some('l'));
    assert_eq!(stream.take(), Some('d'));
    assert_eq!(stream.take(), Some('!'));
    assert_eq!(stream.take(), None); // No more characters
}

#[test]
fn test_mark_and_reset() {
    let input = "Hello, world!";
    let mut stream = BufferedCharStream::new(Cursor::new(input)).unwrap();

    stream.mark();
    assert_eq!(stream.take(), Some('H'));
    assert_eq!(stream.take(), Some('e'));

    stream.reset();
    assert_eq!(stream.take(), Some('H')); // Should return to marked position
}

#[test]
fn test_rollback() {
    let input = "Hello, world!";
    let mut stream = BufferedCharStream::new(Cursor::new(input)).unwrap();
    assert_eq!(stream.take(), Some('H'));
    assert_eq!(stream.take(), Some('e'));
    stream.rollback(2); // Rollback 1 character
    assert_eq!(stream.take(), Some('H')); // Should return to 'H' after rollback
}

#[test]
fn test_has_remaining() {
    let input = "Hello";
    let mut stream = BufferedCharStream::new(Cursor::new(input)).unwrap();

    assert!(stream.has_remaining());
    stream.take(); // Consume one character
    assert!(stream.has_remaining());
    stream.take(); // Consume more characters
    stream.take();
    stream.take();
    stream.take();
    assert!(!stream.has_remaining()); // No remaining characters
}

#[test]
fn test_line_number() {
    let input = "First line\nSecond line\nThird line";
    let mut stream = BufferedCharStream::new(Cursor::new(input)).unwrap();
    assert_eq!(stream.line(), 1); // Start at line 1
    stream.take();
    stream.take();
    stream.take();
    stream.take();
    stream.take();
    stream.take();
    stream.take();
    stream.take();
    stream.take();
    stream.take();
    stream.take();
    assert_eq!(stream.line(), 2); // Line 2
    stream.take();
    stream.take();
    stream.take();
    stream.take();
    stream.take();
    stream.take();
    stream.take();
    stream.take();
    stream.take();
    stream.take();
    stream.take();
    stream.take();
    assert_eq!(stream.line(), 3); // Line 2
}
