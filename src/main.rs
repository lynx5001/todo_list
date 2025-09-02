mod todo;

use std::io;
use clearscreen;
use todo::{Task, add_new_task, display_todo_list, check_task};

fn main() {

    // create list
    let mut todo_list: Vec<Task> = Vec::new();

    // dummy tasks
    todo_list.push(Task{name: "Running".to_string(), is_checked: false,});
    todo_list.push(Task{name: "Biking".to_string(), is_checked: true,});
    todo_list.push(Task{name: "Homework".to_string(), is_checked: false,});
    todo_list.push(Task{name: "Cooking".to_string(), is_checked: true,});
    todo_list.push(Task{name: "Cleaning dishes".to_string(), is_checked: false,});
    todo_list.push(Task{name: "Emails".to_string(), is_checked: false,});

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
            "s" => display_todo_list(&todo_list, false),
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
            "c" => {
                let mut name = String::new();

                // ask user for task name
                println!("\nEnter task name you want to check: ");
                io::stdin().read_line(&mut name).expect("Failed to read task name!");    

                check_task(&mut todo_list, name);
            },
            "q" => {break;},
            "h" => display_todo_list(&todo_list, true),
            _ => println!("\nWork in progress, option not available!"), 
        }
    }
    println!("\nThank you for using rust todo list!");
}
