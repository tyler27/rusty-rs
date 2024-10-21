use common::document::span::Span;

#[test]
fn test_from_vec() {
    let spans: Vec<Span> = vec![Span::new(1, 2), Span::new(3, 4)];
    let span: Span = Span::from_vec(spans);
    assert_eq!(span, Span::new(1, 4))
}
