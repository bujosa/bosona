#[derive(Debug)]
pub struct Task {
    pub title: String,
    pub done: bool,
}

// Builder Pattern for Task struct
impl Task {
    pub fn new(title: impl Into<String>) -> Task {
        Task {
            title: title.into(),
            done: false,
        }
    }
}
