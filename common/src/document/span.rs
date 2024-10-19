#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    begin: i32,
    end: i32,
}

impl Span {
    pub fn new(begin: i32, end: i32) -> Self {
        Span { begin, end }
    }

    pub fn from_vec(spans: Vec<Span>) -> Self {
        let begin = spans.iter().map(|s| s.begin).min().unwrap_or(0);
        let end = spans.iter().map(|s| s.end).max().unwrap_or(0);
        Span { begin, end }
    }

    pub fn add(&mut self, spans: &[Span]) {
        for span in spans {
            self.add_span(span);
        }
    }

    pub fn add_span(&mut self, span: &Span) {
        self.begin = self.begin.min(span.begin);
        self.end = self.end.max(span.end);
    }

    pub fn contains(&self, position: i32) -> bool {
        position >= self.begin && position <= self.end
    }
}
