


use std::{env, fs::{File, OpenOptions}, io::{self, Write}};
fn main() -> io::Result<()>{
    // collecting arguments from CLI
    let arg:Vec<String> = env::args().skip(1).collect();

    //if arguments are there, check if first argument is "add"
    if arg.len() > 0 && arg[0] == "add".to_string(){
        writing_tasks(arg[1..].join(" "))?;
    }
    Ok(())
}
fn writing_tasks(task: String) -> io::Result<()>{
    // converting input to a vec of &[u8] to write in a local file
    let byte_task = task.as_bytes().to_vec();
    
    // looking for "Tasks.txt" file
    let file_read = File::open("Tasks.txt");
    match file_read{
        // Ok if file was found, append to the file operation proceed
        Ok(_file) => {
            let mut file = OpenOptions::new()
                .append(true)
                .open("Tasks.txt")?;
            let prefix_data: &[u8] = b"\n"; 
            let mut combined_args = prefix_data.to_vec();
            combined_args.extend_from_slice(&byte_task);
            
            file.write_all( &combined_args)?;
            println!("File updated successfully: Tasks.txt");
        },
        // if file "Task.txt" was not found, creating a new file.
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
            let mut file = File::create("Tasks.txt")?;
            file.write_all(&byte_task)?;
            println!("New File created successfully: Tasks.txt");
        },
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return Err(e);
        }
    }
    Ok(())
}