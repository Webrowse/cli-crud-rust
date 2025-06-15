use crate::list;
use std::io;


pub fn complete(task_str: String) -> io::Result<()> {
    let target_task = match task_str.parse::<usize>() {
        Ok(num) if num > 0 => num,
        _ => {
            eprintln!("Please provide valid task number");
            return Ok(());
        }
    };
    // The argument number's new name is "target_task" is a 'usize'

    let file_content = match list::list_tasks() {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading from File");
            return Err(e);
        }
    };
    // file_content have all the data of the file in String format in line by line tasks.

    let mut lines: Vec<String> = file_content.lines().map(|line| line.to_string()).collect();
    // Each line of file_content was collected as an separate element in a vector names "lines".

    let mut task_found_count ; //Because undex starts with 0, and list starts with 1

    for (i, line) in lines.iter_mut().enumerate() {
        //iter through the Vector with tasks
        if !line.is_empty() {
            //skipping through the empty lines if any.
            task_found_count = i + 1; //to keep up the lagg of 1 

            if task_found_count == target_task {
                //when the number and task number matches
                if line.contains("[ ]") {
                    *line = line.replace("[ ]", "[X]");
                    println!("Task {} marked as complete", task_found_count);
                    break;
                } else {
                    *line = line.replace("[X]", "[ ]");
                    println!("Task {} marked as incomplete", task_found_count);
                    break;
                }
            }
        }
    }
    let content = lines.join("\n");
    let new_content_n = format!("{}\n",content);
    let mut file = File::create("Tasks.txt")?;
    file.write_all(new_content_n.as_bytes())?;

    Ok(())
}

