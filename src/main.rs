use bosona::task::Task;

fn main() {
    // Default Pattern
    let task = Task::default();
    println!("Default {:?}", task);

    // Or
    let task: Option<Task> = None;
    let task = task.unwrap_or_default();
    println!("Unwrap {:?}", task);

    // Constructor Pattern
    let task = Task::new("Learn Rust");
    println!("{:?}", task);
}
