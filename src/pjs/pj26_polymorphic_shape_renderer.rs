use colored::Colorize;
use std::f64::consts::PI;
use std::io::{Write, stdin, stdout};

trait Shape {
    fn name(&self) -> &str;
    fn area(&self) -> f64;
}
struct Triangle {
    base: f64,
    height: f64,
}
impl Shape for Triangle {
    fn name(&self) -> &str {
        "Triangle"
    }
    fn area(&self) -> f64 {
        (self.height * self.base) / 2_f64
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}
impl Shape for Rectangle {
    fn name(&self) -> &str {
        "Rectangle"
    }
    fn area(&self) -> f64 {
        self.height * self.width
    }
}
struct Circle {
    radius: f64,
}
impl Shape for Circle {
    fn name(&self) -> &str {
        "Circle"
    }
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

pub fn polymorphic_shape() {
    let mut shapes: Vec<Box<dyn Shape>> = Vec::new();
    loop {
        println!(
            "\t\n1. Add Circle\t\n2. Add Rectangle\t\n3. Add Triangle\t\n4. Show All Areas\t\n5. Exit"
        );
        let mut input = prompt("choice");
        match input.as_str() {
            "1" => {
                let radius = if let Ok(r) = prompt("radius").parse::<f64>() {
                    r
                } else {
                    println!("{}", "enter a valid radius".bright_red());
                    continue;
                };
                shapes.push(Box::new(Circle { radius }))
            }
            "2" => {
                let width = if let Ok(w) = prompt("width").parse::<f64>() {
                    w
                } else {
                    println!("{}", "enter a valid width".bright_red());
                    continue;
                };
                let height = if let Ok(h) = prompt("height").parse::<f64>() {
                    h
                } else {
                    println!("{}", "enter a valid height".bright_red());
                    continue;
                };
                shapes.push(Box::new(Rectangle { width, height }))
            }
            "3" => {
                let base = if let Ok(b) = prompt("base").parse::<f64>() {
                    b
                } else {
                    println!("{}", "enter a valid base".bright_red());
                    continue;
                };
                let height = if let Ok(h) = prompt("radius").parse::<f64>() {
                    h
                } else {
                    println!("{}", "enter a valid height".bright_red());
                    continue;
                };
                shapes.push(Box::new(Triangle { base, height }))
            }
            "4" => {
                for (i , shape ) in shapes.iter().enumerate() {
                    println!("{}- {} area is {:.2}", i+1 , shape.name(), shape.area())
                }

            },
            "5"=> break,
            _ => continue,
        }

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
