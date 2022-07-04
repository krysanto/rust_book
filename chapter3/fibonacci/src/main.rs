use std::io;

fn main() {
    let mut fibonacci_to = String::new();

    io::stdin()
        .read_line(&mut fibonacci_to)
        .expect("Failed to read line");

    let fibonacci_to: u32 = match fibonacci_to.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    let mut fibonacci = fibonacci(fibonacci_to);

    println!("The number to the {} fibonacci number is {}", fibonacci_to, fibonacci);
}

fn fibonacci (number: u32) -> u32 {
    if number <= 2{
        return 1;
    }

    return (fibonacci(number - 1)) + (fibonacci(number - 2));
}