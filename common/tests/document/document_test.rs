use common::document::span::Span;

#[test]
fn test_document_clone() {
    let span = Span::new(1, 5);
    let cloned_span = span.clone();
    assert_eq!(cloned_span, span); // Check that cloned span is equal to the original
}

#[test]
fn test_spans_from_vec() {
    let span = Span::from_vec(vec![Span::new(1, 5), Span::new(5, 100)]);
    let cloned_span = span.clone();
    for i in 1..100 {
        assert_eq!(cloned_span.contains(i), true);
    }
}
