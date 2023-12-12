use std::io::{stdin, stdout, Write};

pub fn calculate_tip_and_total() {
    let bill = read_user_input("What is the bill? ");
    let tip_percentage = read_user_input("What is the tip percentage? ");
    let tip = bill * tip_percentage / 100.0;
    let total = bill + tip;
    println!("The tip is ${:.2}", tip);
    println!("The total is ${:.2}", total);
}

fn read_user_input(prompt: &str) -> f64 {
    let mut user_input = String::new();
    print!("{}", prompt);
    stdout().flush().unwrap();
    stdin().read_line(&mut user_input).unwrap();
    user_input.trim().parse::<f64>().unwrap()
}
