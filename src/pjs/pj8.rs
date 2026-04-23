pub fn fibonacci(n: u32) -> (Vec<u32>,Vec<u32>, Vec<u32>) {
    let mut fib_vec = Vec::new();
    let mut odd_v = Vec::new();
    let mut even_v = Vec::new();

    for i in 0..=n {
        match i {
            0 => {
                fib_vec.push(0);
                even_v.push(0)
            }
            1 => {
                fib_vec.push(1);
                odd_v.push(1)
            }
            _ => {
                let next_val = fib_vec[i as usize - 1] + fib_vec[i as usize - 2];
                fib_vec.push(next_val);
                if next_val % 2 == 0 {
                    even_v.push(next_val)
                } else {
                    odd_v.push(next_val)
                }
            }
        }
    }

    (fib_vec,even_v, odd_v)
}
#[test]
fn test() {
    println!(" {:?}", fibonacci(8))
}
