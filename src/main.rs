use std::io;
use clearscreen;

// create task struct
struct Task {
    name: String,
    is_checked: bool,
    // other properties like note, list type, deadline, priority...
}

fn add_new_task(mut p_todo_list: Vec<Task>, p_new_task: Task) -> Vec<Task> {

    //add task to todo list and return todo list
    p_todo_list.push(p_new_task);
    p_todo_list
}

// // edit existing task
// fn edit_task(){

// }

// // check task on list
// fn check_task(){

// }

// display current todo list
fn display_todo_list(p_todo_list: &Vec<Task>) {

    println!{"Todo-List: "}
    println!{"--------------------"}
    for task in p_todo_list {
        if !task.is_checked {
            println!("{}", task.name);
        }
    }
}

fn main() {

    // create list
    let mut todo_list: Vec<Task> = Vec::new();
    clearscreen::clear().expect("Failed to clear screen.");

    // welcome message
    println!("Welcome to your todo-list powered by rust!\n");
    
    loop{
        let mut option = String::new();

        // menu for todo-list
        println!("\nSelect option for you todo-list: ");
        println!("s - show current todo-list");
        println!("a - add task to todo-list");
        println!("e - edit task on todo-list");
        println!("c - check task on todo-list");
        println!("h - see checked tasks");
        println!("q - quit application");


        // take user input
        io::stdin().read_line(&mut option).expect("Failed to read option!");    
        // switch case to activate function 
        match option.trim() {
            "s" => display_todo_list(&todo_list),
            "a" => {
                // create new task
                let mut new_task = Task {
                    name: String::new(),
                    is_checked: false,
                };

                // ask user for task name
                println!("\nEnter task name: ");
                io::stdin().read_line(&mut new_task.name).expect("Failed to read task name!");    

                todo_list = add_new_task(todo_list, new_task);
            },
            "q" => {break;},
            _ => println!("\nWork in progress, option not available!"), 
        }
    }
    println!("\nThank you for using rust todo list!");
}
