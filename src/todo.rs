use crate::task::Task;

pub struct ToDoList {
    tasks: Vec<Task>,
}

impl ToDoList {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, description: String) {
        let id = self.tasks.len() + 1;
        let task = Task::new(id, description);
        self.tasks.push(task);
    }

    pub fn complete_task(&mut self, id: usize) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.mark_complete();
        } else {
            println!("Task with ID {} not found.", id);
        }
    }

    pub fn show_tasks(&self) {
        for task in &self.tasks {
            println!(
                "[{}] {} - {}",
                task.id,
                task.description,
                if task.completed { "Completed" } else { "Pending" }
            );
        }
    }
}
