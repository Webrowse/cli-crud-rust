


use std::{env, fs::{File, OpenOptions}, io::{self, Write}};
fn main() -> io::Result<()>{
    let arg:Vec<String> = env::args().skip(1).collect();
    if arg[0] == "add".to_string(){
        // let task = arg[1..].join(" ");
        writing_tasks(arg[1..].join(" "))?;
    }
    // println!("New Task added: {:?}",tasks);
    Ok(())
}
fn writing_tasks(task: String) -> io::Result<()>{
    let file_read = File::open("Tasks.txt");
    match file_read{
        Ok(_file) => {
            let mut file = OpenOptions::new().append(true).open("Tasks.txt")?;
            file.write_all( task.as_bytes())?;
            println!("File updated successfully: Tasks.txt");
        },
        Err(_e) => {
            let mut file = File::create("Tasks.txt")?;
            file.write_all(task.as_bytes())?;
            println!("New File created successfully: Tasks.txt");
        }
    }
    Ok(())
}