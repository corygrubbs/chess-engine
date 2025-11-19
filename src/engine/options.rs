pub struct Options {
    pub debug: bool
}

impl Default for Options {
    fn default() -> Self {
        Self { debug: false }
    }
}