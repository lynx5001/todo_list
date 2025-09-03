mod todo;

use std::io;
use clearscreen;
use todo::{Task, add_new_task, display_todo_list, check_task, display_task_details, edit_task};

fn main() {

    // create list
    let mut todo_list: Vec<Task> = Vec::new();

    // dummy tasks
    todo_list.push(Task{name: "Running".to_string(), is_checked: false, note: "".to_string()});
    todo_list.push(Task{name: "Biking".to_string(), is_checked: true, note: "".to_string()});
    todo_list.push(Task{name: "Homework".to_string(), is_checked: false, note: "Do Maths and logic exercise sheet".to_string()});
    todo_list.push(Task{name: "Cooking".to_string(), is_checked: true, note: "".to_string()});
    todo_list.push(Task{name: "Cleaning dishes".to_string(), is_checked: false, note: "".to_string()});
    todo_list.push(Task{name: "Emails".to_string(), is_checked: false, note: "Answer emails from work and check spam".to_string()});

    clearscreen::clear().expect("Failed to clear screen.");

    // welcome message
    println!("Welcome to your todo-list powered by rust!\n");
    
    loop{
        let mut option = String::new();

        // menu for todo-list
        println!("\nSelect option for you todo-list: ");
        println!("s - show current todo-list");
        println!("a - add task to todo-list");
        println!("d - show task details");
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
                let mut task = Task {
                    name: String::new(),
                    is_checked: false,
                    note: String::new(),
                };

                // ask user for task name
                println!("\nEnter task name: ");
                io::stdin().read_line(&mut task.name).expect("Failed to read task name!"); 
                println!("\nAdd description for task: ");  
                io::stdin().read_line(&mut task.note).expect("Failed to read task description!"); 
                task.name = task.name.trim().to_string();
                task.note = task.note.trim().to_string();
                todo_list = add_new_task(todo_list, task);
            },
            "c" => {
                let mut name = String::new();

                // ask user for task name
                println!("\nEnter task name you want to check: ");
                io::stdin().read_line(&mut name).expect("Failed to read task name!");    
                name = name.trim().to_string();
                check_task(&mut todo_list, name);
            },
            "d" => {
                let mut name = String::new();
                let mut name_copy = String::new();
                let mut continue_option = String::new();

                // ask user for task name
                println!("\nEnter task name to see details: ");
                io::stdin().read_line(&mut name).expect("Failed to read task name!");
                name = name.trim().to_string();
                name_copy = name.clone();
                display_task_details(&todo_list, name);

                println!("e - edit task");
                println!("r - return to menu");
                io::stdin().read_line(&mut continue_option).expect("Failed to read option!");
                continue_option = continue_option.trim().to_string();

                if continue_option == "e" {
                    // create new task
                    let mut edited_task = Task {
                        name: String::new(),
                        is_checked: false,
                        note: String::new(),
                    };

                    // ask user for task name
                    println!("\nEnter new task name: ");
                    io::stdin().read_line(&mut edited_task.name).expect("Failed to read task name!"); 
                    println!("\nAdd new description for task: ");  
                    io::stdin().read_line(&mut edited_task.note).expect("Failed to read task description!"); 
                    edited_task.name = edited_task.name.trim().to_string();
                    edited_task.note = edited_task.note.trim().to_string();

                    edit_task(&mut todo_list, name_copy, &edited_task);
                }
            },
            "q" => {break;},
            "h" => display_todo_list(&todo_list, true),
            _ => println!("\nWork in progress, option not available!"), 
        }
    }
    println!("\nThank you for using rust todo list!");
}
