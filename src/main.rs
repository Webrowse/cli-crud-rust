use std::{
    env,
    fs::OpenOptions,
    io::{self, Write},
};

mod list;
mod delete;
mod complete;
fn main() -> io::Result<()> {
    // collecting arguments from CLI
    let arg: Vec<String> = env::args().skip(1).collect();
    
    //if arguments are there, check if first argument is "add"
    if arg.len() > 0 {
        
        if arg[0] == "add".to_string() {
            writing_tasks(arg[1..].join(" "))?;
            println!("{}",list::list_tasks().expect("error"));
        }
        if arg[0] == "list".to_string() {
            let content = list::list_tasks()?;
            for (index, value) in content.lines()
                .filter(|x| !x.trim().is_empty())
                .enumerate() {
                println!("{}. {}", index + 1, value.trim());
            }
            
        }
        if arg[0] == "complete".to_string() {
            complete::complete(arg[1].clone())?;
            println!("{}",list::list_tasks().expect("Complete ran into trouble"));
        }
        
        if arg[0] == "delete".to_string() {
            delete::delete(arg[1].clone())?;
            println!("{}",list::list_tasks().expect("delete ran into trouble"));
        }
    };
    
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




