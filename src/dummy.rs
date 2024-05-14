use std::{env, thread, time::Duration};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <process_name>", args[0]);
        return;
    }

    // Simulate long-running process
    loop {
        thread::sleep(Duration::from_secs(3600));
    }
}

