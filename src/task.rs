#[derive(Debug)]
pub struct Task {
    pub id: usize,
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn new(id: usize, description: String) -> Self {
        Self {
            id,
            description,
            completed: false,
        }
    }

    pub fn mark_complete(&mut self) {
        self.completed = true;
    }
}
