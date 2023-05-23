#[derive(Debug)]
pub struct Task {
    pub title: String,
    pub done: bool,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            title: String::new(),
            done: false,
        }
    }
}

// Constructor Pattern
impl Task {
    pub fn new(title: impl Into<String>) -> Task {
        Task {
            title: title.into(),
            done: false,
        }
    }
}
