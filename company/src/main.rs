use std::{io::{self, BufRead}, collections::HashMap};

fn main() {
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();

    let stdin = io::stdin();

    println!("Type 'Add <name> to <department>' to add an employee");

    println!("Type 'List <department>' to list the employees of a department");

    println!("Type 'All' to list all employees by department");

    println!("Type 'Quit' to quit");

    for line in stdin.lock().lines() {
        let input = line.expect("error: unable to read user input");

        match Command::from_input(&input) {
            Some(Command::Add {dept, name}) => employees.entry(dept).or_default().push(name),
            Some(Command::List(dept)) => match employees.get(&dept) {
                Some(names) => {
                    for name in names {
                        println!("{}: {}", dept, name);
                    }
                }
                None => println!("I don't recognize that department"),
            },
            Some(Command::All) => {
                for (dept, names) in &employees {
                    let mut names = names.clone();

                    names.sort();

                    for name in names {
                        println!("{}: {}", dept, name);
                    }
                }
            },
            Some(Command::Quit) => break,
            None => println!("Input error"),
        }
    }
    
    println!("Have a nice day!");
}

enum Command {
    Add { dept: String, name: String },
    List(String),
    All,
    Quit,
}

impl Command {
    fn from_input(s: &str) -> Option<Self> {
        let words: Vec<&str> = s.trim().split_whitespace().collect();

        match words.as_slice() {
            ["All"] => Some(Command::All),
            ["Quit"] => Some(Command::Quit),
            ["List", dept] => Some(Command::List(String::from(*dept))),
            ["Add", name, "to", dept] => Some(Command::Add {
                    dept: String::from(*dept),
                    name: String::from(*name),
                }),
            _ => None,
        }
    }
}
