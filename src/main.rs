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

        match input.trim().parse().to_str() {
            "y" => break true,
            "n" => break false,
            _ => println!("Please enter y or n."),
        }
    }
}

fn main() {
    // Ask "Is the car silent when you turn the key?" (Yes - "Are the battery terminals corroded?" (Yes - "Clean terminals and try starting again.") | (No - "Replace cables and try again.") ) | (No - "Does the car make a clicking noise?" (Yes - "Replace the battery.") | (No - "Does the car crank up but fail to start?" (Yes - "Check spark plug connections.") | (No - "Does the engine start and then die?" (Yes - "Does your car have fuel injection?" (Yes - "Get it in for service.") | (No - "Check to ensure the choke is opening and closing.")))))
}
