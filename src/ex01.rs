use std::io::{stdin, stdout, Write};

pub fn tip_calculator() {
    let mut input = String::new();
    print!("What is the bill? ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();
    let bill = input.trim().parse::<f64>().unwrap();

    let mut input = String::new();
    print!("What is the tip percentage? ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();
    let tip_percentage = input.trim().parse::<f64>().unwrap();

    let tip = bill * tip_percentage / 100.0;
    let total = bill + tip;

    println!("The tip is ${:.2}", tip);
    println!("The total is ${:.2}", total);
}
