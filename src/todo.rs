use crate::task::Task;

pub fn add_new_task(mut p_todo_list: Vec<Task>, p_new_task: Task) -> Vec<Task> {

    //add task to todo list and return todo list
    p_todo_list.push(p_new_task);
    p_todo_list
}

pub fn edit_task(p_todo_list: &mut Vec<Task>, p_name: String, p_edited_task: &Task) {
    for task in p_todo_list.iter_mut() {
        println!("search for task to change");
        if task.name == p_name {
            println!("changing task");
            task.name = p_edited_task.name.clone();
            task.note = p_edited_task.note.clone();
            break;
        }
    }
}

pub fn check_task(p_todo_list: &mut Vec<Task>, p_name: String) {
    // iterate over task and check the wanted task
    for task in p_todo_list.iter_mut() {
        if task.name == p_name {
            task.is_checked = !task.is_checked;
        }
    }
}

pub fn delete_task(p_todo_list: &mut Vec<Task>, p_name: String) {
    p_todo_list.retain(|task| task.name != p_name);
}

// display task details
pub fn display_task_details(p_todo_list: &Vec<Task>, p_name: String){
    // iterate over task and display the task details
    for task in p_todo_list {
        if task.name == p_name {
            println!("\nTask {} \n-------------------- \nChecked: {}, \nNote: {}", task.name, task.is_checked.to_string(), task.note);
        }
    }
}

pub fn display_todo_list(p_todo_list: &Vec<Task>, show_checked_tasks: bool) {

    // checked tasks
    if show_checked_tasks {
        println!{"\nChecked Tasks: "}
        println!{"--------------------"}
        for task in p_todo_list {
            if task.is_checked {
                println!("[x] {}", task.name);
            }
        }
    // todo list
    } else {
        println!{"\nTodo-List: "}
        println!{"--------------------"}
        for task in p_todo_list {
            if !task.is_checked {
                println!("[ ] {}", task.name);
            }
        }
    }
}
