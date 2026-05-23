//Copies large files in parallel using the rayon crate,
// splitting files into 1MB chunks processed by worker threads that write to pre-allocated target files.

// first thoughts:
// defining the critical sections : clearly the file
// how do I slice the file into chunks
// do we need thread pool , when file chunks exceed number of cores , this will become overhead, so we do need a thread pool
// each worker responsible for a chunk at a time
// how do we define states , finished , started , failed
// handling permissions
// how do we construct the file again , we must make sure all chunks arrived

// initial flow :
// read the path
// split into chunks
// load in memory (this is not ideal i think, what if file chunks exceed available memory , we must make sure chunk size never exceed)
// lock the chunks till all arrived (locking them where ? in memory not ideal at all , what if there is available memory for just one or two chunks )
// we split chunks and save them in block storage instead of memory with custom format or extension
// reconstruct the file and delete the chunks

// final flow
// open source file
// get the size of file
// allocate destination with that size
// calculate the chunks needed
// each thread :
//   get chunk offset
//   read source file
//   write dest file

use rayon::prelude::*;

use std::fs::{File, OpenOptions};
use std::io;
use std::io::ErrorKind::NotFound;
use std::io::{Read, Seek, SeekFrom, Write, stdin, stdout};
use std::path::PathBuf;
use std::sync::Arc;

static CHUNK_SIZE: usize = 1; // 1MB
pub fn file_copier() -> io::Result<()> {
    let file_path = get_input("enter file you want to copy: ")?;
    let file_name = file_path.file_name().unwrap();
    let mut output_path = get_input("enter directory you want to copy to: ")?;
    output_path.push(file_name);
    let file = File::open(&file_path)?;
    let file_size = file.metadata()?.len() as usize;
    // round up
    let chunks = file_size.div_ceil(CHUNK_SIZE);
    // if there is no previous files with same name it will be created , unless it will be overwritten
    let mut copy_file = OpenOptions::new()
        .truncate(true)
        .write(true)
        .create(true)
        .open(&output_path)?;
    copy_file.set_len(file_size as u64)?;
    let src_arc = Arc::new(file_path);
    let dest_arc = Arc::new(output_path);

    (0..chunks).into_par_iter().try_for_each(|i| {
        let offset = i * CHUNK_SIZE;
        let mut buffer = vec![0u8; CHUNK_SIZE.min(file_size - offset)];
        let mut file = File::open(&*src_arc)?;
        file.seek(SeekFrom::Start(offset as u64))?;
        file.read_exact(&mut buffer)?;
        let mut copy_file = OpenOptions::new().write(true).open(&*dest_arc)?;
        copy_file.seek(SeekFrom::Start(offset as u64))?;
        copy_file.write_all(&buffer)?;
        Ok::<(), io::Error>(())
    })?;
    Ok(())
}
fn get_input(message: &str) -> Result<PathBuf, io::Error> {
    print!("{message}");
    stdout().flush()?;
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let trimmed = input.trim().to_string();
    let path = PathBuf::from(trimmed);
    if path.exists() {
        println!();
        Ok(path)
    } else {
        Err(io::Error::new(NotFound, "path does not exist"))
    }
}
