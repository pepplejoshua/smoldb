use std::io::Write;

enum MetaCommandResult {
    Success,
    Unrecognized,
    Quit,
}

enum Statement {
    Insert,
    Select,
    Unknown(String),
}

use MetaCommandResult::*;
use Statement::*;

fn show_prompt() {
    print!("smoldb> ");
    std::io::stdout().flush().unwrap(); // makes sure the prompt is printed to the temrinal
}

pub fn eval_loop() {
    loop {
        show_prompt();
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                let trimmed_input = input.trim();
                // check if the first character is a '.'
                // which would mean a meta command
                match trimmed_input.chars().next() {
                    Some(ch) => {
                        if ch == '.' {
                            // meta command
                            match execute_meta_command(&trimmed_input.to_string()) {
                                Success => {
                                    println!("entered: {trimmed_input}");
                                    continue;
                                }
                                Unrecognized => {
                                    println!("unrecognized meta command: {trimmed_input}");
                                    continue;
                                }
                                Quit => {
                                    println!("exiting...");
                                    break;
                                }
                            }
                        } else {
                            // statement
                            let statement = prepare_statement(&trimmed_input.to_string());
                            exec_statement(&statement);
                        }
                    }
                    None => continue,
                }
            }
            Err(err) => {
                println!("input error: {:}", err);
                break;
            }
        };
    }
}

fn execute_meta_command(input: &String) -> MetaCommandResult {
    if input == ".q" {
        Quit
    } else {
        Unrecognized
    }
}

fn prepare_statement(input: &String) -> Statement {
    if input.starts_with("insert") {
        Insert
    } else if input.starts_with("select") {
        Select
    } else {
        Unknown(input.clone())
    }
}

fn exec_statement(statement: &Statement) {
    match statement {
        Insert => println!("we will implement insert here"),
        Select => println!("we will implement select here"),
        Unknown(input) => println!("unknown statement: {input}"),
    }
}
