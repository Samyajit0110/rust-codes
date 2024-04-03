use std::io;

fn is_prime(num: u32) -> bool {
    if num <= 1 {
        return false;
    }
    for i in 2..=(num / 2) {
        if num % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    println!("Enter a number:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    if is_prime(input) {
        println!("{} is a prime number.", input);
    } else {
        println!("{} is not a prime number.", input);
    }
}

