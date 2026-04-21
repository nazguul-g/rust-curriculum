use std::io::{Write, stdin, stdout};
use std::process::{Command, ExitStatus};
use std::{io};

pub fn temp_converter() -> io::Result<()> {
    let mut string = String::new();
    println!("what do u want to convert?");
    println!("\t A)- Celsius to Fahrenheit.");
    println!("\t B)- Fahrenheit to Celsius.");
    print!("select an option (A/B): ");
    stdout().flush()?;
    stdin().read_line(&mut string)?;
    match string.trim().to_lowercase().as_str() {
        "a" => {
            clear_cmd()?;
            convert_c_to_f()?;
        }
        "b" => {
            clear_cmd()?;
            convert_f_to_c()?;
        }
        _ => println!("please choose valid option"),
    }

    Ok(())
}
fn convert_c_to_f() -> io::Result<f64> {
    let mut s = String::new();

    print!("enter the value: ");
    stdout().flush()?;
    stdin().read_line(&mut s)?;
    let value = match s.trim().to_lowercase().parse::<f64>() {
        Ok(val) => (val * 1.8) + 32.00,
        Err(_) => {
            println!("please enter a valid number");
            0.0
        }
    };
    println!("the value is {:?}", value);

    Ok(value)
}
fn convert_f_to_c() -> io::Result<f64> {
    let mut s = String::new();

    print!("enter the value: ");
    stdout().flush()?;
    stdin().read_line(&mut s)?;
    let value = match s.trim().to_lowercase().parse::<f64>() {
        Ok(val) => (val - 32.0) / 1.8,
        Err(_) => {
            println!("please enter a valid number");
            0.0
        }
    };
    Ok(value)
}
fn clear_cmd() -> io::Result<ExitStatus> {
    if cfg!(target_os = "windows") {
        println!("{}",cfg!(windows));
        Command::new("cmd").args(["/c", "cls"]).status()
    } else {

        Command::new("clear").status()

    }
}
#[test]
fn test() -> io::Result<()> {
    temp_converter()
    // you cant use test to this function (infinite loop) , cuz teste doesnt have stdin
}
