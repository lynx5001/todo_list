// #[derive(Debug, Clone)]
// create task struct
pub struct Task {
    pub name: String,
    pub is_checked: bool,
    pub note: String,
    // type: String,
    // deadline: u8,
    // priority: String,
}

impl Task {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), is_checked: false, note: String::new() }
    }
}