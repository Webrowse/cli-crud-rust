use std::{
    env,
    fs::{File, OpenOptions},
    io::{self, Read, Write},
};
fn main() -> io::Result<()> {
    // collecting arguments from CLI
    let arg: Vec<String> = env::args().skip(1).collect();

    //if arguments are there, check if first argument is "add"
    if arg.len() > 0 {
        if arg[0] == "add".to_string() {
            writing_tasks(arg[1..].join(" "))?;
        }
        if arg[0] == "list".to_string() {
            let content = list_tasks()?;
            for (index, value) in content.lines().filter(|x| !x.trim().is_empty()).enumerate() {
                println!("{}. {}", index + 1, value.trim());
            }
        }
        if arg[0] == "complete".to_string() {
            complete(arg[1].clone())?;
        }
    }
    Ok(())
}
fn writing_tasks(task: String) -> io::Result<()> {
    let formatted_task = format!("[ ] {} \n", task);
    let combined_args = formatted_task.as_bytes();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("Tasks.txt")?;

    file.write_all(&combined_args)?;
    println!("File updated successfully: Tasks.txt");

    Ok(())
}

fn list_tasks() -> io::Result<String> {
    let mut file = File::open("Tasks.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn complete(task_str: String) -> io::Result<()> {
    let target_task = match task_str.parse::<usize>() {
        Ok(num) if num > 0 => num,
        _ => {
            eprintln!("Please provide valid task number");
            return Ok(());
        }
    };
    // The argument number's new name is "target_task" is a 'usize'

    let file_content = match list_tasks() {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading from File");
            return Err(e);
        }
    };
    // file_content have all the data of the file in String format in line by line tasks.

    let mut lines: Vec<String> = file_content.lines().map(|line| line.to_string()).collect();
    // Each line of file_content was collected as an separate element in a vector names "lines".

    let task_found_count; //Because undex starts with 0, and list starts with 1

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
                } else {
                    *line = line.replace("[X]", "[ ]");
                    println!("Task {} marked as incomplete", task_found_count);
                }
            }
        }
        break;
    }
    let new_content = lines.join("\n");

    let mut file = File::create("Tasks.txt")?;
    file.write_all(new_content.as_bytes())?;

    Ok(())
}
