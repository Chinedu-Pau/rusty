
use std::fs::OpenOptions;
use std::io::{self, Write};

fn read_input(prompt: &str) -> String //This makes the function 
//to give back a string data type.
  { 
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    let mut contents = String::new();

    loop {
        let task_name = read_input("\nEnter a task_name (or 'done' to finish): ");
        if task_name.to_uppercase() == "DONE" {
            break;
        }

        let mut tasks: Vec<String> = Vec::new();
        loop {
            let task = read_input("Add a task (or 'done' for the next task_name): ");
            if task.to_uppercase() == "DONE" {
                break;
            }
            tasks.push(task);
        }

        contents.push_str(&format!("{}:\n", task_name));
        for i in 0..tasks.len() {
            contents.push_str(&format!("  {}. {}\n", i + 1, tasks[i]));
        }
    }

    let mut file = OpenOptions::new().append(true).open("tasks.txt").expect("could not create file");
    file.write_all(contents.as_bytes()).expect("write failed");
    println!("Saved to tasks.txt");
}
