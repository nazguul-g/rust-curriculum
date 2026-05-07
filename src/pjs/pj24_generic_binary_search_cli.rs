// cli interface, prompts the user to enter number or word to search it in a hardcoded vectors
// implement the binary search algo for numbers
// apply generic rules to it
// add partial order functionality

use std::io::{Write, stdin, stdout};

// the input
fn prompt(message: &str) -> String {
    print!("{}", message);
    let _ = stdout().flush();
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("error parsing the input");
    input.trim().to_string()
}
pub fn bsearch_cli() {
    let _: Vec<i32> = (0..50).map(|i| i * 5).collect();
    let _: Vec<&str> = vec!["nazguul", "banana", "apple", "orange", "fruit"];
}
// generic function
fn binary_search <T: std::cmp::PartialEq + std::cmp::PartialOrd> (array: &[T], target: &T) -> Option<usize> {
    if array.is_empty() {
        return None;
    }

    let mut min = 0;
    let mut max = array.len() - 1;

    while min <= max {
        let mid = min + (max - min) / 2;
        if *target == array[mid] {
            return Some(mid);
        } else if array[mid] > *target {
            max = mid - 1;
        } else {  // array[mid] < *target
            min = mid + 1;
        }
    }
    None
}

#[test]
fn test() {
    let number_vec: Vec<i32> = (0..50).map(|i| i * 5).collect();
    println!("{:?}", number_vec);
    //let vector = vec![1, 2, 3, 4, 5];
    let mut string_vec: Vec<&str> = vec!["nazguul", "banana", "apple", "orange", "fruit"];
    string_vec.sort();
    println!("{:?}", string_vec);

    println!("{:?}", binary_search(&number_vec, &245));
    println!("{:?}", binary_search(&string_vec, &"banana"));
}
