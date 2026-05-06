use colored::Colorize;
use regex::Regex;
use std::io::{Write, stdin, stdout};
pub fn data_validation_util() {
    let phone_regex = Regex::new(r"^\+?[0-9]{10,15}$").unwrap();
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    loop {
        println!("\nwhat do u want to check:\n\t1-password\n\t2-email\n\t3-phone number\n\t4-exit");
        let input = prompt("enter your option: ");
        match input.as_str() {
            "1" => password_check(),
            "2" => email_check(&email_regex),
            "3" => phones_check(&phone_regex),

            "4" => break,
            _ => continue,
        }
    }
}

fn prompt(message: &str) -> String {
    print!("{}", message);
    let _ = stdout().flush();
    let mut input = String::new();

    stdin()
        .read_line(&mut input)
        .expect("error reading the input");
    input = input.trim().to_string();
    input
}
fn phones_check(pattern: &Regex) {
    let phone = prompt("enter your phone number (e,g,. +213553951941): ");
    if pattern.is_match(&phone) {
        println!("{}", "valid phone number".bright_blue())
    } else {
        println!("{}", "phone number not valid".bright_red())
    }
}
fn email_check(pattern: &Regex) {
    let phone = prompt("enter your email (e,g,. email@example.com): ");
    if pattern.is_match(&phone) {
        println!("{}", "valid email".bright_green())
    } else {
        println!("{}", "email not valid".bright_red())
    }
}

fn password_check() {
    let pwd = prompt("enter your password: ");
    if password_strongness(&pwd) {
        println!("{}", "valid password".bright_green())
    }else {
        println!("{}", "password not valid".bright_red())
    }
}

fn password_strongness(pwd: &str) -> bool {
    if pwd.len() < 8 {
        return false;
    }
    let has_uppercase = pwd.chars().any(|c| c.is_ascii_uppercase());
    let has_lowercase = pwd.chars().any(|c| c.is_ascii_lowercase());
    let has_digit = pwd.chars().any(|c| c.is_ascii_digit());
    let has_special = pwd.chars().any(|c| c.is_ascii_alphanumeric());
    has_uppercase && has_special && has_digit && has_digit && has_lowercase
}
