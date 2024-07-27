use std::io;

fn main() {
    println!("Please enter the position of the Fibonacci sequence you want to calculate:");

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        let n: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid integer.");
                continue;
            }
        };

        let fibonacci_number = fibonacci_binet(n);
        println!("The Fibonacci number at position {n} is {fibonacci_number}");
        break;
    }
}

fn fibonacci_binet(n: u32) -> f64 {
    let sqrt_5 = 5_f64.sqrt();
    let phi = (1_f64 + sqrt_5) / 2_f64;
    let psi = (1_f64 - sqrt_5) / 2_f64;

    ((phi.powf(n as f64) - psi.powf(n as f64)) / sqrt_5).round()
}
