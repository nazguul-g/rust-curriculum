mod closures;
pub mod dynamic_dispatch;

// fn once means the function its useable once and nver allowed again
fn fnonce() {
    let message = String::from("Self-destruct sequence initiated");

    // This closure is FnOnce because it 'moves' message
    let burner_phone = move || {
        let consumed = message; // message is moved here
        println!("Message: {}", consumed);
    }; // consumed (and message) are dropped here

    burner_phone(); // This works perfectly.

    //burner_phone();
    // ^ ERROR: "use of moved value: `burner_phone`"
    // The compiler knows the "candle" has already been burned.
}