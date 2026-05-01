use chrono::Local;
use std::collections::HashMap;
use std::sync::LazyLock;

pub static RESPONSES: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert(
        "what is your name".to_string(),
        "I'm Rusty, your friendly Rust-powered chatbot! 🤖".to_string(),
    );
    map.insert(
        "who are you".to_string(),
        "I'm your personal Rust assistant here to help with programming and motivation!"
            .to_string(),
    );
    map.insert(
        "motivation".to_string(),
        "Remember: Every expert was once a beginner. Keep coding and you'll get there! 💪"
            .to_string(),
    );
    map.insert(
        "inspire me".to_string(),
        "The only way to do great work is to love what you do. - Steve Jobs ✨".to_string(),
    );
    map.insert(
        "where are you from".to_string(),
        "I was born in the digital cloud, powered by Rust's amazing ecosystem! ☁️".to_string(),
    );
    map.insert(
        "what do you think about rust".to_string(),
        "Rust is revolutionary! It gives you:\n- Memory safety without GC\n- Fearless concurrency\n- Blazing fast performance\n- An amazing community 🦀".to_string()
    );
    map.insert(
        "how to learn rust".to_string(),
        "Here's the best path:\n1. Read 'The Book' (official Rust documentation)\n2. Do Rustlings exercises\n3. Build small projects\n4. Contribute to open-source\n5. Never stop learning! 📚".to_string()
    );
    map.insert(
        "best rust features".to_string(),
        "My favorite Rust features are:\n- Ownership system\n- Pattern matching\n- Zero-cost abstractions\n- Cargo package manager\n- Helpful compiler messages".to_string()
    );
    map.insert(
        "rust vs python".to_string(),
        "Both are great! Rust gives you performance and safety, while Python offers simplicity. Use Rust for:\n- System programming\n- Performance-critical code\n- Concurrency\nUse Python for:\n- Rapid prototyping\n- Data science\n- Scripting".to_string()
    );
    map.insert(
        "thank you".to_string(),
        "You're very welcome! Happy to help. Keep rocking with Rust! 🦀".to_string(),
    );
    map.insert(
        "bye".to_string(),
        "Goodbye! Remember: The only bad code is the code you don't write! 🚀".to_string(),
    );
    map.insert(
        "time".to_string(),
        format!("Current time is {}", Local::now().format("%H:%M")),
    );
    map.insert(
        "date".to_string(),
        format!("Today is {}", Local::now().format("%Y-%m-%d")),
    );
    map.insert(
        "who created you".to_string(),
        "I was created by a Rust enthusiast to demonstrate Rust's capabilities for building CLI applications!".to_string()
    );
    map.insert(
        "joke".to_string(),
        "Why did the Rustacean break up with Python?\nBecause it wanted to borrow, not reference! 😄".to_string()
    );
    map.insert(
        "tip".to_string(),
        "Pro tip: Always run `cargo clippy` - it's like having a senior Rust developer looking over your shoulder!".to_string()
    );
    map.insert("hello".to_string(), "Hello there 🤖".to_string());
    map.insert(
        "what's up|hi|hey".to_string(),
        "What's happening?".to_string(),
    );
    map.insert(
        "about rust|tell me about rust".to_string(),
        "Rust is a systems programming language...".to_string(),
    );
    map.insert(
        "help|commands|menu".to_string(),
        "I can discuss Rust, Linux, blockchain, or just chat! Try asking about 'ownership' or 'arch'.".to_string()
    );
    map.insert(
        "arch linux|tell me about arch".to_string(),
        "Arch is a rolling-release distro for those who want full control. Keep your neofetch ready! 🐧".to_string()
    );
    map.insert(
        "ownership|borrowing".to_string(),
        "Memory safety without a GC! Just remember: one mutable reference or multiple immutable ones. 🦀".to_string()
    );
    map.insert(
        "cargo|package manager".to_string(),
        "Cargo is Rust's build system. It handles compiling, downloading libraries, and building docs effortlessly.".to_string()
    );
    map.insert(
        "blockchain|voting system".to_string(),
        "Secure, transparent, and decentralized. Perfect for a graduation thesis project! ⛓️"
            .to_string(),
    );
    map.insert(
        "smart pointers|box|rc|arc".to_string(),
        "Boxes for heap allocation, Rc for reference counting, and Arc for thread-safe sharing! 🧠"
            .to_string(),
    );
    map.insert(
        "neovim|nvim|vim".to_string(),
        "The ultimate editor for speed. Keep tweaking that init.lua until it's perfect! ⌨️"
            .to_string(),
    );
    map.insert(
        "hyprland|tiling manager".to_string(),
        "Dynamic tiling with Wayland. Nothing beats a smooth, blur-filled workspace. ✨"
            .to_string(),
    );
    map.insert(
        "git|github|version control".to_string(),
        "Always commit early and often. Don't forget to sync your Obsidian vault too! 🔄"
            .to_string(),
    );
    map.insert(
        "clippy|linting".to_string(),
        "Listen to Clippy! It’s the best way to learn idiomatic Rust and avoid common pitfalls."
            .to_string(),
    );
    map.insert(
        "concurrency|threads".to_string(),
        "Fearless concurrency! Rust ensures you don't have data races at compile time. 🧵"
            .to_string(),
    );
    map.insert(
        "error handling|result|option".to_string(),
        "In Rust, we don't do nulls. We use Option for presence and Result for success/failure! ⚠️"
            .to_string(),
    );
    map
});
