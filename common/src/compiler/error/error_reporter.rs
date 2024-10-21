use std::sync::Mutex;

/// Represents a compilation error.
#[derive(Debug, Clone)]
pub struct CompilerError {
    message: String,
    // Other fields can be added as needed
}

impl CompilerError {
    pub fn new(message: &str) -> Self {
        CompilerError {
            message: message.to_string(),
        }
    }
}

/// Holds a list of all the errors reported during the compilation process.
pub struct ErrorReporter {
    /// A list of all the errors that have been reported.
    errors: Mutex<Vec<CompilerError>>, // Using Mutex for thread safety
}

impl ErrorReporter {
    /// Creates a new `ErrorReporter` instance.
    pub fn new() -> Self {
        ErrorReporter {
            errors: Mutex::new(Vec::new()),
        }
    }

    /// Adds a new error to the errors list of the reporter.
    ///
    /// # Arguments
    ///
    /// * `error` - The error to add to the errors list.
    pub fn add_error(&self, error: CompilerError) {
        let mut errors = self.errors.lock().unwrap(); // Lock the mutex
        errors.push(error);
    }

    /// Returns an immutable reference to the list of reported errors.
    pub fn get_errors(&self) -> Vec<CompilerError> {
        self.errors.lock().unwrap().clone() // Clone to return an immutable list
    }
}
