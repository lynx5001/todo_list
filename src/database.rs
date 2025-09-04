use crate::task::Task;

use csv::Writer;
use csv::ReaderBuilder;
use std::error::Error;

const csv_path: &str = "todo_list.csv";

pub fn write_todo_list(p_todo_list: Vec<Task>) -> Result<(), Box<dyn Error>> {
    // creating csv handler
    let writer_result = Writer::from_path(csv_path);
    let mut writer = match writer_result {
        Ok(writer) => writer,
        Err(err) => return Err(Box::new(err)),
    };

    //write header
    writer.write_record(&["name", "is_checked", "note"]);

    //iterate over the records
    for task in p_todo_list {
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

pub fn read_todo_list(flag_ignore_error: bool) -> Result<Vec<Task>, Box<dyn Error>> {
    let mut todo_list: Vec<Task> = Vec::new();

    let reader_result = ReaderBuilder::new().from_path(csv_path);
    let reader = match reader_result {
        Ok(reader) => reader,
        Err(err) => return Err(Box::new(err)),
    };

    for record in reader.into_records() {
        let record = match record {
            Ok(record) => record,
            Err(err) => {
                if flag_ignore_error {
                    continue;
                } else {
                    return Err(Box::new(err));
                }
            }
        };

        // assign the current record to a Task in the todo-list
        let task_string: Vec<String> = record
            .iter()
            .map(|field| field.to_string())
            .collect();
        let [name, is_checked, note]: [String; 3] = task_string.try_into().expect("wrong length");
        let task: Task = Task{name: name, is_checked: is_checked.parse()?, note: note};

        todo_list.push(task);
    }

    Ok(todo_list)
}