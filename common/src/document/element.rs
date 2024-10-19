use crate::document::span::Span;

trait Element {
    fn get_span() -> Span;
}
