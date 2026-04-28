use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};


pub fn file_reader() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 4 {
        eprintln!("usage: cargo run <file_path> [--lines] [--search] <keyword>");
        return;
    }
    let show_lines =  args.contains(&"--lines".to_string());
    let path = &args[1];
    // getting the keyword from the arguments ,
    // the issue raised here is the position of the keyword isn't fixed ,it can change
    let keyword = match args.iter().position(|x| x == "--search") {
        Some(pos) => args.get(pos + 1),
        None => None
    };
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("failed to open the file in {}, {}", path, e);
            return;
        }
    };
    let reader = BufReader::new(file);
    for (i , line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let matched = keyword.map_or(true, |k| line.contains(k));
        if matched {
            if show_lines {
                println!("{}, {}", i+1,line)
            }else {

                println!("{}",line)

            }
        }
    }

}
