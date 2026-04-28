use std::io::{Write, stdin, stdout};

pub fn string_manipulate() {
    println!("------string manipulation tool-----");

    loop {
        println!("choose an operation");
        println!(
            "\n\t1-Reverse\n\t2-Uppercase\n\t3-Lowercase\n\t4-Trim\n\t5-Find substring\n\t6-Replace Text\n\t7-Quit the app"
        );
        print!("input: ");
        let _ = stdout().flush();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "1" => {
                let input = prompt("enter the string");
                let reversed = &input.chars().rev().collect::<String>();
                println!("the reverse of \"{}\" is : {}", input, reversed)
            }
            "2" => {
                let input = prompt("enter the string");
                let uppercased = &input.to_uppercase();
                println!("the uppercase of \"{}\" is : {}", input, uppercased)
            }
            "3" => {
                let input = prompt("enter the string");
                let lowercase = &input.to_lowercase();
                println!("the lowercase of \"{}\" is : {}", input, lowercase)
            }
            "4" => {
                let input = prompt("enter the string");
                let trimmed = &input.trim();
                println!(" \"{}\" trim result is : {}", input, trimmed)
            }
            "5" => {
                let input = prompt("enter the string");
                let find = prompt("enter the substring u wanna find");
                let found = input.contains(&find);
                if found {
                    println!("the substring \"{}\" in \"{}\" . is found", find, input)
                } else {
                    println!("the substring \"{}\" in \"{}\" . isn't found", find, input)
                }
            }
            "6" => {
                let input = prompt("enter the string: ");
                let remove = prompt("enter the substring u wanna replace in the string :");
                let replace = prompt("enter what your are want to replace it with :");
                let final_string = input.replace(&remove,&replace);
                println!("the final string is {}"  , final_string)

            }
            "8" => break,
            _ => {
                println!("please enter a valid option from above ");
                continue;
            }
        }
    }
}
fn prompt(message: &str) -> String {
    print!("{message}: ");
    let _ = stdout().flush();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
