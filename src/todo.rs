
// #[derive(Debug, Clone)]
// create task struct
pub struct Task {
    pub name: String,
    pub is_checked: bool,
    // note: String,
    // type: String,
    // deadline: u8,
    // priority: String,
}

impl Task {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), is_checked: false }
    }
}

pub fn add_new_task(mut p_todo_list: Vec<Task>, p_new_task: Task) -> Vec<Task> {

    //add task to todo list and return todo list
    p_todo_list.push(p_new_task);
    p_todo_list
}

pub fn check_task(p_todo_list: &mut Vec<Task>, p_name: String) {
    // iterate over task and check the wanted task
    for task in p_todo_list.iter_mut() {
        if task.name == p_name {
            task.is_checked = true;
        }
    }
}

// // edit existing task
// fn edit_task(){

// }

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
