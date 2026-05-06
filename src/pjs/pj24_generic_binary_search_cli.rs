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
    let number_vec: Vec<i32> = (0..50).map(|i| i * 5).collect();
    let string_vec: Vec<&str> = vec!["nazguul", "banana", "apple", "orange", "fruit"];
}
// generic function
fn binary_search<T>(vector: &[T], target: T) -> (bool, usize) {
    let mut min = 0;
    let mut max = vector.len();

    while min < max {
        let mid = min + (max - min) / 2;

        if vector[mid] == target {
            return (true, mid);
        } else if vector[mid] < target {
            min = mid + 1;
        } else {
            max = mid;
        }
    }

    (false, 0)
}

#[test]
fn test () {
    let number_vec: Vec<i32> = (0..50).map(|i| i * 5).collect();
    println!("{:?}",number_vec);
    //let vector = vec![1, 2, 3, 4, 5];
    let string_vec: Vec<&str> = vec!["nazguul", "banana", "apple", "orange", "fruit"];
    println!("{:?}",string_vec);

    println!("{:?}", binary_search(&number_vec, 1));
    println!("{:?}", binary_search(&string_vec, "banana"));


}
