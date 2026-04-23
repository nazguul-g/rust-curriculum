use std::io;
use std::io::{Write, stdin, stdout};

#[derive(Debug)]
enum BodyStatus {
    Underweight,
    Normal,
    Overweight,
    Obese,
}
// height by meters , (e,g,. 1.80 meter)
// weight by kilograms , (e,g,. 80 kg)
struct Index {
    height: f32,
    weight: f32,
    age: u8,
    is_male: bool,
}
pub fn bmi_calculator() -> io::Result<()> {
    let mut index: Index = Index {
        height: 0.0,
        weight: 0.0,
        age: 0,
        is_male: false,
    };

    println!("body mass index calculator");
    loop {
        let mut s = String::new();
        print!("your age ? =");
        stdout().flush()?;
        stdin().read_line(&mut s)?;
        match s.trim().parse::<u8>() {
            Ok(num) => index.age = num,
            Err(_) => {
                println!("thats not a valid age, try again");
                continue;
            }
        }
        let mut s = String::new();
        print!("your height  ? (e,g,. 1.80 meter) =");
        stdout().flush()?;
        stdin().read_line(&mut s)?;
        match s.trim().parse::<f32>() {
            Ok(num) => index.height = num,
            Err(_) => {
                println!("thats not a valid height, try again");
                continue;
            }
        }

        let mut s = String::new();
        print!("your weight  ? (e,g,. 80.5 kilograms) =");
        stdout().flush()?;
        stdin().read_line(&mut s)?;
        match s.trim().parse::<f32>() {
            Ok(num) => index.weight = num,
            Err(_) => {
                println!("thats not a valid weight, try again");
                continue;
            }
        }
        let mut s = String::new();
        print!("are u male of female=");
        stdout().flush()?;
        stdin().read_line(&mut s)?;
        match s.trim().to_lowercase().as_str() {
            "male" => index.is_male = true,
            "female" => index.is_male = false,
            _ => {
                println!("thats not a valid input, try again");
                continue;
            }
        }
        break;
    }
    let bmi = index.weight / (index.height.powf(2.0));
    let body_estimation = match bmi {
        x if x < 18.5 => BodyStatus::Underweight,
        18.5..=24.9 => BodyStatus::Normal,
        25.0..=29.9 => BodyStatus::Overweight,
        _ => BodyStatus::Obese,
    };
    let gender = match index.is_male {
        true => 1,
        false => 0,
    };
    let bfp = (1.20 * bmi) + (0.23 * index.age as f32) - (10.8 * gender as f32) - 5.4;
    println!(
        "your body BMI is , {:?} . with overral body fat {:.1}",
        body_estimation, bfp
    );
    Ok(())
}
