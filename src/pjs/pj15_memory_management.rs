use std::rc::Rc;
use std::cell::RefCell;

pub fn memory_mgmt() {
    println!("🧠 Memory Management Demo in Rust");

    // Ownership
    let s1 = String::from("Ownership Example");
    let s2 = s1; // s1 moved
    println!("🔑 Ownership transferred: {}", s2);
    // println!("{}", s1); // ❌ Error: s1 moved

    // Borrowing
    let s3 = String::from("Borrowing Example");
    borrow_demo(&s3);
    println!("✅ After borrow: {}", s3); // still accessible

    // Mutable Borrowing
    let mut s4 = String::from("Hello");
    mutate_demo(&mut s4);
    println!("🔧 After mutation: {}", s4);

    // Lifetimes
    let result;
    let a = String::from("abcd");
    {
        let b = String::from("xyz");
        result = longest(&a, &b);
        println!("⏳ Longest string: {}", result);
    }

    // Box (heap allocation)
    let boxed = Box::new(42);
    println!("📦 Boxed value: {}", boxed);

    // Rc (reference-counted pointer)
    // allow multiple references point at same data , with keep counter if counter reaches 0 (no one using it ) the pointer is dropped 
    let rc_val = Rc::new(String::from("Shared"));
    let rc_clone = Rc::clone(&rc_val);
    println!("📚 Rc values: {}, {}", rc_val, rc_clone);
    println!("Ref count: {}", Rc::strong_count(&rc_val));

    // RefCell (interior mutability)
    // so this means even if we dont mutate   , the refceel still can change , and makes the checks at runtime not compile time
    let cell = RefCell::new(100);
    // We just changed the data inside an immutable variable.
    *cell.borrow_mut() += 50;
    println!("🧪 RefCell value: {}", cell.borrow());
}

fn borrow_demo(data: &String) {
    println!("📥 Borrowed: {}", data);
}

fn mutate_demo(data: &mut String) {
    data.push_str(" World");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
