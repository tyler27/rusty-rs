trait IdManager {
    /// Finds the associated script id by the file name and extension.
    ///
    /// If none is found, one will be created and assigned to that script.
    ///
    /// Returns the script_id
    fn find_or_create_script_id(&mut self, name: &str, extension: &str) -> i32;

    /// Finds the associated script id by the file name and extension.
    ///
    /// If none is found, the compiler will panic.
    ///
    /// Returns the script_id
    fn find_script(&self, name: &str, extension: &str) -> i32;
}
