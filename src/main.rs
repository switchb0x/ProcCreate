use std::process::Command;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file_path = "processes.txt";
    let file = File::open(file_path).expect("Could not open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let process_name = line.expect("Could not read line");

        // Spawn the dummy process with a specific name argument
        let child = Command::new("./target/release/dummy")
            .arg(&process_name)
            .spawn()
            .expect("Failed to start dummy process");

        // Optionally, keep track of child processes to manage them later
        println!("Started dummy process with name: {}", process_name);

        // You may choose not to wait on each child so they run concurrently
    }
}

