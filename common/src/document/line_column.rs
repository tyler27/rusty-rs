#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineColumn {
    /// The line number within the document.
    line: i32,
    /// The column number within the line.
    column: i32,
}

impl LineColumn {
    /// The LineColumn object with the minimum position.
    pub const MIN: LineColumn = LineColumn { line: 0, column: 0 };

    /// The LineColumn object with the maximum position.
    pub const MAX: LineColumn = LineColumn {
        line: i32::MAX,
        column: i32::MAX,
    };

    /// Creates a new LineColumn.
    pub fn new(line: i32, column: i32) -> Self {
        LineColumn { line, column }
    }

    /// Checks whether the given position is lesser than this position.
    pub fn is_lesser_than(&self, other: &LineColumn) -> bool {
        if self.line == other.line {
            self.column < other.column
        } else {
            self.line < other.line
        }
    }

    /// Checks whether the given position is greater than this position.
    pub fn is_greater_than(&self, other: &LineColumn) -> bool {
        if self.line == other.line {
            self.column > other.column
        } else {
            self.line > other.line
        }
    }
}
