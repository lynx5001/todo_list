use crate::task::Task;

use csv::Writer;
use std::error::Error;

pub fn write_todo_list (p_todo_list: Vec<Task>) -> Result<(), Box<dyn Error>> {
    println!("starting write csv");
    // creating csv handler
    let writer_result = Writer::from_path("todo_list.csv");
    let mut writer = match writer_result {
        Ok(writer) => writer,
        Err(err) => return Err(Box::new(err)),
    };

    //write header
    writer.write_record(&["name", "is_checked", "note"]);

    //iterate over the records
    for task in p_todo_list {
        println!("iterate and add tasks");
        let record_result = match writer.write_record(&[
            task.name, 
            task.is_checked.to_string(), 
            task.note
            ]) {
        Ok(record_result) => record_result,
        Err(err) => return Err(Box::new(err)),            
     };
    }
    Ok(())
}

// pub fn read_todo_list () {

// }