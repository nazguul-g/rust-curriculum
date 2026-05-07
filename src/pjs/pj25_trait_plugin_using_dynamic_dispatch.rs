use std::io::{Write, stdin, stdout};
use colored::Colorize;

trait Plugin {
    fn name(&self) -> String;
    fn run(&self, input: &str) -> String;
}
struct UpperCase;
impl Plugin for UpperCase {
    fn name(&self) -> String {
        "Upper Case".to_string()
    }
    fn run(&self, input: &str) -> String {
        input.to_uppercase()
    }
}
struct LowerCase;
impl Plugin for LowerCase {
    fn name(&self) -> String {
        "Lower Case".to_string()
    }
    fn run(&self, input: &str) -> String {
        input.to_lowercase()
    }
}
struct Duplicate;
impl Plugin for Duplicate {
    fn name(&self) -> String {
        "Duplicate".to_string()
    }
    fn run(&self, input: &str) -> String {
        format!("{}{}", input, input)
    }
}
struct Reverse;
impl Plugin for Reverse {
    fn name(&self) -> String {
        "Reverse".to_string()
    }
    fn run(&self, input: &str) -> String {
        input.chars().rev().collect()
    }
}
pub fn plugin() {
    // we wont create for each options its own object
    //let uppercase = UpperCase;
    // uppercase.run()
    // we do this instead
    // this wont work cuz the plugin wont fit in the stack , its a heap DS
    //let plugins:Vec<dyn Plugin>;
    // this as well wont work cuz of Plugin size is unknown at compiler time , rust always must know size of things to align them perfectly
    //let vector:Vec<Box<Plugin>> ;
    let plugins:Vec<Box<dyn Plugin>> = vec![Box::new(UpperCase),Box::new(Reverse),Box::new(LowerCase),Box::new(Duplicate)];
    loop {

        println!("available plugins: ");
        for (i , plugin) in plugins.iter().enumerate() {
            println!("\t{}- {}", i+1, plugin.name())
        }
        println!("\t{}- Exit", plugins.len()+1);
        let input = prompt("your option");
        let option = match input.parse::<usize>() {
            Ok(num) if num>=1 && num < plugins.len()=> num-1,
            Ok(num) if num == plugins.len()+1=> {
                break
            },
           _ => continue

        };
        let data = prompt("enter your string");
        let result = plugins[option].run(&data);
        println!("{} of {} is {}", plugins[option].name(),data, result.bright_blue())


    }
}
fn prompt(message: &str) -> String {
    print!("{message}: ");
    let _ = stdout().flush();
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("failed to parse input");
    input.trim().to_string()
}
