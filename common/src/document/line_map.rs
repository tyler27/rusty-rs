pub struct LineMap {
    /// A table that holds the start positions of each line.
    pub table: Vec<usize>,
}

impl LineMap {
    /// Constructs a new LineMap instance from the given text.
    ///
    /// # Arguments
    ///
    /// * `text` - A string slice representing the document text.
    ///
    /// # Returns
    ///
    /// A new LineMap instance.
    pub fn new(chars: &[char]) -> Self {
        let mut buffer = Vec::new();
        buffer.push(0); // The start of the first line

        let mut pos = 0;

        while pos < chars.len() {
            match chars[pos] {
                '\r' => {
                    if pos + 1 < chars.len() && chars[pos + 1] == '\n' {
                        pos += 1; // Skip over '\n' after '\r'
                    }
                    buffer.push(pos + 1); // Record the start of the next line
                }
                '\n' => {
                    buffer.push(pos + 1); // Record the start of the next line
                }
                _ => {}
            }
            pos += 1;
        }

        LineMap { table: buffer }
    }

    /// Returns the offset of the specified line number (1-indexed).
    ///
    /// # Arguments
    ///
    /// * `line` - The line number to retrieve the offset for.
    ///
    /// # Returns
    ///
    /// The offset of the specified line number.
    ///
    /// # Panics
    ///
    /// Panics if the line number is out of range.
    pub fn get_line_offset(&self, line: usize) -> usize {
        if line < 1 || line > self.table.len() {
            panic!(
                "Line number out of range: {} expected [1-{}]",
                line,
                self.table.len()
            );
        }
        self.table[line - 1]
    }

    /// Returns the line number of the character at the specified offset.
    ///
    /// # Arguments
    ///
    /// * `offset` - The offset to retrieve the line number from.
    ///
    /// # Returns
    ///
    /// The line number starting from 1.
    pub fn get_line_number(&self, offset: usize) -> usize {
        self.table
            .binary_search(&offset)
            .map_or_else(|pos| pos, |line| line + 1) // Adjust for 1-indexing
    }
}
