// Reads and filters CSV data using the csv crate, matching values in a specified column and printing matching rows to standard output.
// first thoughts
// who handles file read/write ? is it the crate or buffers
// how the csv is structured
// how do we search
// why we dont just use file descriptors to just search for what we want

// csv is structured , header and data entries . columns and rows , first row is a header , the following are entries
// first we prompt for column , user enter which column he wants to search defining from header , them we search the entries
// csv deal with csv better then file operations,  we cant parse properly the structure of csv using just them
// the csv have its own reader and writer

// whe query found , we highlight it with green color

use std::error::Error;
use std::fs::OpenOptions;
use std::io::{Read, Write, stdin, stdout};
use std::path::Path;

pub fn csv() -> Result<(), Box<dyn Error>> {
    loop {
        let path = get_input("file path: ");
        let path = Path::new(&path);
        if !path.exists() && !path.extension().unwrap().eq("csv") {
            println!("file not found");
            continue;
        }
        let header = get_input("name of column: ");
        let search_for = get_input("search key: ");
        // let file = OpenOptions::new().read(true).open(path)?;
        // let mut csv = csv::Reader::from_reader(file);
        let mut csv = csv::Reader::from_path(path)?;
        let col_index = csv
            .headers()?
            .iter()
            .position(|h| h == header)
            .ok_or("column not found")?;
        let records = csv.records();
        for record in records {
            let record = record?;
            if record.get(col_index) == Some(search_for.as_str()) {
                println!("{}", record.iter().collect::<Vec<_>>().join(","));
            }
        }
        break
    }

    Ok(())
}

fn get_input(message: &str) -> String {
    print!("{message}");
    let _ = stdout().flush();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    input
}
