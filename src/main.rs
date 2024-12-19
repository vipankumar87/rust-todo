mod task;
mod todo;
mod utils;

use todo::ToDoList;
use utils::print_divider; // Importing the function

fn main() {
    let mut todo_list = ToDoList::new();
    
    todo_list.add_task("Learn Rust".to_string());
    todo_list.add_task("Build a to-do list app".to_string());
    todo_list.add_task("Contribute to open source".to_string());
    print_divider();

    println!("Current To-Do List:");
    todo_list.show_tasks();
    
    todo_list.complete_task(1);
    
    println!("\nTo-Do List after completing a task:");
    todo_list.show_tasks();
}
