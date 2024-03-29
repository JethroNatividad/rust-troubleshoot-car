use std::io;
use std::io::Write;

// Write a program to troubleshoot a car, follow decision tree.
// Inputs: Inputs to the decision tree, y or n
// Process: process the decision tree
// Outputs: the output of the decision tree

fn get_decision(prompt: &str) -> bool {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim() {
            "y" => return true,
            "n" => return false,
            _ => println!("Please enter y or n."),
        }
    }
}

fn main() {
    match get_decision("Is the car silent when you turn the key? (y/n): ") {
        true => match get_decision("Are the battery terminals corroded? (y/n): ") {
            true => println!("Clean terminals and try starting again."),
            false => println!("Replace cables and try again."),
        },
        false => match get_decision("Does the car make a clicking noise? (y/n): ") {
            true => println!("Replace the battery."),
            false => match get_decision("Does the car crank up but fail to start? (y/n): ") {
                true => println!("Check spark plug connections."),
                false => match get_decision("Does the engine start and then die? (y/n): ") {
                    true => match get_decision("Does your car have fuel injection? (y/n): ") {
                        true => println!("Get it in for service."),
                        false => println!("Check to ensure the choke is opening and closing."),
                    },
                    false => println!("Get it in for service."),
                },
            },
        },
    }
}
