use std::io::{self, Write};

pub fn prime_number() -> io::Result<()> {
    let mut input_s = String::new();
    println!("Welcome to the prime number checker");

    let number = loop {
        print!("Enter a number to check: ");
        io::stdout().flush()?;
        input_s.clear();
        io::stdin().read_line(&mut input_s)?;
        match input_s.trim().parse::<usize>() {
            Ok(num) => {
                if num < 0 {
                    println!("Invalid input, please enter a valid number.");
                    continue;
                } else {
                    break num;
                }
            }

            Err(_) => {}
        }
    };
    println!("{}", is_primal(number));
    println!("{:?}",primes_vector(number));

    Ok(())
}
fn is_primal (number :usize) -> bool {

    match number {
        x if x < 2 => false,
        2 => true,
        x if x % 2 == 0 => false,
        /*       _ => {
            let mut p = true;
            for i in (3..=number.isqrt()).step_by(2) {
                if number % i == 0 {
                    p = false
                }
            }
            p
        }*/
        _ => {
            let has_devisor = (3..=number.isqrt()).step_by(2).any(|n| number % n == 0);
            !has_devisor
        }
    }
}
fn primes_vector(number: usize) -> Vec<usize> {
    let mut vector = Vec::new();
    for i in 0..=number {
        if is_primal(i) {
            vector.push(i)
        }
    }

    vector
}