use std::io::{Write, stdin, stdout};

pub fn calculator() {
    println!("welcome to a simple calculator");
    print!("please enter the equation (e,g,. 3 + 4 , 5 / 7): ");
    let mut input = String::new();
    stdout().flush().expect("please retry");
    stdin()
        .read_line(&mut input)
        .expect("err reading the input, please retry");

    let tokens: Vec<&str> = input.split_whitespace().collect();
    println!("{:?}",tokens);
    let result = match tokens[1] {
        "+" => tokens[0].parse::<f64>().unwrap() + tokens[2].parse::<f64>().unwrap(),
        "/" => {
            if tokens[2].parse::<f64>().unwrap() == 0.0 {
                println!("division by zero");
                -1.0
            } else {
                tokens[0].parse::<f64>().unwrap() / tokens[2].parse::<f64>().unwrap()
            }
        }

        "*" => tokens[0].parse::<f64>().unwrap() * tokens[2].parse::<f64>().unwrap(),
        "-" => tokens[0].parse::<f64>().unwrap() - tokens[2].parse::<f64>().unwrap(),
        _ => {
            println!("error calculating");
            -1.0
        }
    };
    println!("the result is : {}",result)
}
