fn shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace()
        .min_by_key(|word| word.len())
}

fn main() {
    println!("Enter a string of words:");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    let input = input.trim(); // Remove newline character at the end

    match shortest_word(input) {
        Some(shortest) => println!("The shortest word is: {}", shortest),
        None => println!("No words found in the input"),
    }
}

