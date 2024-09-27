use std::io;

fn main() {
    println!("Insert the first number");
    let mut first = String::new();

    io::stdin()
        .read_line(&mut first)
        .expect("Failed to read line");

    let first: f64 = first.trim().parse().expect("Please type a number");

    println!("Insert the operator");

    let mut operator = String::new();

    io::stdin()
        .read_line(&mut operator)
        .expect("Failed to read line");

    let operator = operator.trim();

    println!("Insert the second number");

    let mut second = String::new();

    io::stdin()
        .read_line(&mut second)
        .expect("Failed to read line");

    let second: f64 = second.trim().parse().expect("Please type a number");

    match operator {
        "+" => println!("The result is {}", first + second),
        "-" => println!("The result is {}", first - second),
        "*" => println!("The result is {}", first * second),
        "/" => println!("The result is {}", first / second),
        _ => println!("Invalid operator"),
    }
}
