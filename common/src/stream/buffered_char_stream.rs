use crate::document::line_map::LineMap;
use std::io::{self, BufReader, Read};
use std::str;

pub struct BufferedCharStream {
    line_map: LineMap,
    buffer: Vec<char>,
    pos: usize,
    mark: Option<usize>,
}

impl BufferedCharStream {
    /// Constructs a new BufferedCharStream from the given input stream.
    ///
    /// # Arguments
    ///
    /// * `stream` - The input stream to read characters from.
    ///
    /// # Returns
    ///
    /// A new BufferedCharStream instance.
    pub fn new<R: Read>(stream: R) -> io::Result<Self> {
        let buffer = Self::read_all_chars(stream)?;
        let line_map = LineMap::new(&buffer); // Create LineMap immediately
        Ok(BufferedCharStream {
            line_map,
            buffer,
            pos: 0,
            mark: None,
        })
    }

    /// Constructs a new BufferedCharStream from a character buffer.
    ///
    /// # Arguments
    ///
    /// * `buffer` - A vector of characters to use as the buffer.
    ///
    /// # Returns
    ///
    /// A new BufferedCharStream instance.
    pub fn from_buffer(buffer: Vec<char>) -> Self {
        let line_map = LineMap::new(&buffer); // Create LineMap immediately
        BufferedCharStream {
            line_map,
            buffer,
            pos: 0,
            mark: None,
        }
    }

    /// Reads all characters from the input stream into a Vec<char>.
    ///
    /// # Arguments
    ///
    /// * `stream` - The input stream to read from.
    ///
    /// # Returns
    ///
    /// A vector of characters read from the stream.
    fn read_all_chars<R: Read>(stream: R) -> io::Result<Vec<char>> {
        let mut reader = BufReader::new(stream);
        let mut buffer = Vec::new();
        let mut byte_buffer = Vec::new();

        // Read bytes and convert to UTF-8 string
        reader.read_to_end(&mut byte_buffer)?;
        let content = str::from_utf8(&byte_buffer).expect("Invalid UTF-8");

        // Collect characters into the buffer
        buffer.extend(content.chars());
        Ok(buffer)
    }

    /// Takes the next character from the stream.
    pub fn take(&mut self) -> Option<char> {
        if self.pos >= self.buffer.len() {
            return None;
        }
        let ch = self.buffer[self.pos];
        self.pos += 1;
        Some(ch)
    }

    /// Peeks at the next character without consuming it.
    pub fn peek(&self) -> Option<char> {
        if self.pos >= self.buffer.len() {
            return None;
        }
        Some(self.buffer[self.pos])
    }

    /// Marks the current position in the stream.
    pub fn mark(&mut self) {
        self.mark = Some(self.pos);
    }

    /// Resets the position to the last marked position.
    pub fn reset(&mut self) {
        if let Some(mark) = self.mark {
            self.pos = mark;
        } else {
            panic!("The stream has no marker set");
        }
    }

    /// Rollbacks the position by a specified count.
    pub fn rollback(&mut self, count: usize) {
        if count > self.pos {
            self.pos = 0;
        } else {
            self.pos -= count;
        }
    }

    /// Checks if there are remaining characters in the buffer.
    pub fn has_remaining(&self) -> bool {
        self.pos < self.buffer.len()
    }

    /// Returns the current position in the stream.
    pub fn position(&self) -> usize {
        self.pos
    }

    /// Returns the current line number the stream is at.
    pub fn line(&self) -> usize {
        self.line_map.get_line_number(self.pos)
    }
}
