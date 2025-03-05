use std::io;

// Function to calculate the factorial of a number
fn factorial(n: u32) -> u32 {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}

fn main() {
    // Prompt the user for input
    println!("Enter a number:");

    // Read input from the user
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Convert the input to a u32 number
    let num: u32 = input.trim().parse().expect("Please enter a valid number");

    // Calculate the factorial
    let result = factorial(num);

    // Print the result
    println!("The factorial of {} is {}", num, result);
}
