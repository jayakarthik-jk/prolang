use prolang::common::diagnostics::Diagnostics;
use prolang::interpretation::interpretate;

use std::io::stdin;
use std::io::Write;

fn main() {
    let stdin = stdin();
    print!("Enter the mode: ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    if let std::cmp::Ordering::Equal = input.trim().cmp(&"file".to_string()) {
        file_mode();
        return;
    }
    if let std::cmp::Ordering::Equal = input.trim().cmp(&"".to_string()) {
        file_mode();
        return;
    }
    if let std::cmp::Ordering::Equal = input.trim().cmp(&"console".to_string()) {
        console_mode();
        return;
    }
}

fn console_mode() {
    let stdin = stdin();
    let mut display_progress = true;
    loop {
        print!("$ ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        if let std::cmp::Ordering::Equal = input.trim().cmp(&"progress".to_string()) {
            display_progress = !display_progress;
            continue;
        }
        if let std::cmp::Ordering::Equal = input.trim().cmp(&"exit".to_string()) {
            break;
        }
        if let std::cmp::Ordering::Equal = input.trim().cmp(&"clear".to_string()) {
            // clear the console
            println!("{}[2J", 27 as char);
            continue;
        }

        if let Err(error) = interpretate(input) {
            println!("{}", error);
        }
        Diagnostics::read_errors().iter().for_each(|error| {
            println!("{}", error);
        });
    }
}

fn file_mode() {
    use std::fs::read_to_string;

    let file_name = "app.prolang";
    let input = match read_to_string(file_name) {
        Ok(input) => input,
        Err(error) => {
            eprintln!("error reading file: {}", error);
            return;
        }
    };

    if let Err(error) = interpretate(input) {
        println!("{}", error);
    }
    Diagnostics::read_errors().iter().for_each(|error| {
        println!("{}", error);
    });
}
