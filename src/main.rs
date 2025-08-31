use std::io;

// create task struct
struct Task {
    name: String,
    is_checked: bool,
    // other properties like note, list type, deadline, priority...
}

// add task to list
fn add_new_task(mut p_todo_list: Vec<Task>) -> Vec<Task> {

    // create new task
    let mut new_task = Task {
        name: String::new(),
        // default not checked
        is_checked: false,
    };

    // ask user for name of task and read input
    println!("Enter name of new task:");
    io::stdin().read_line(&mut new_task.name).expect("failed to read task name");    

    println!("Added {} to the list!", new_task.name);
    //add task to todo list
    p_todo_list.push(new_task);
    

    p_todo_list
}

// // edit existing task
// fn edit_task(){

// }

// // check task on list
// fn check_task(){

// }

// display current todo list
fn display_todo_list(p_todo_list: Vec<Task>) {

    println!{"Current Todo-List: "}
    println!{"--------------------"}
    for task in p_todo_list {
        println!("{}", task.name);
    }
}

fn main() {
    let mut todo_list: Vec<Task> = Vec::new();
    todo_list = add_new_task(todo_list);
    todo_list = add_new_task(todo_list);
    todo_list = add_new_task(todo_list);

    display_todo_list(todo_list);
}
