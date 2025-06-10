use std::env;



fn main() -> Result<(), String>{
    let arg:Vec<String> = env::args().skip(1).collect();
    let mut tasks = vec![];
    if arg[0] == "add".to_string(){
        let task = arg[1..].join(" ");
        tasks.push(task);
    }
    println!("{:?}",tasks);
    Ok(())
}
