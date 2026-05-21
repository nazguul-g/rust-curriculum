use std::fmt::{Display, Formatter};
use std::fs::OpenOptions;
use std::{io, path};
use std::io::{BufReader, BufWriter, Error, Write, stdin, stdout};
use std::path::{Path, PathBuf};
use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::{GzEncoder};

pub enum CompressionError {
    Io(io::Error),
    FileNotFound,
    InvalidChoice,
    InvalidExtension,
}
enum Kind {
    Compress,
    Decompress
}
impl Display for CompressionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CompressionError::InvalidChoice => write!(f, "invalid choice selected"),
            CompressionError::FileNotFound => write!(f, "File not found"),
            CompressionError::InvalidExtension => write!(f, "Invalid file extension"),
            CompressionError::Io(error) => write!(f, "{}", error),
        }
    }
}
impl From<io::Error> for CompressionError {
    fn from(value: Error) -> Self {
        CompressionError::Io(value)
    }
}
pub fn compression_algo() -> Result<(), CompressionError> {
    let output_path= "/mnt/workspace/Projects/ms-projects/assets";
    println!("compression algorithm");
    println!("decide an operation\n\t1-Compress file\n\t2-Decompress fle");
    loop {
        let input = get_input("choice: ")?;
        match input.as_str() {
            "1" => {

                let path = get_valid_path("enter file path to compress", false)?;
                compress(&path, &Path::new(output_path))?
            },
            "2" => {
                 let path = get_valid_path("enter file path to compress", true)?;
                decompress(&path, &Path::new(output_path))?
            },
            _ => {
                println!("invalid input try again");
                continue;
            }
        }
    }
}
fn get_input(message: &str) -> Result<String, CompressionError> {
    print!("{message}");
    stdout().flush()?;
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
fn get_valid_path(prompt: &str, is_output: bool) -> Result<PathBuf, CompressionError> {
    print!("{prompt}");
    stdout().flush()?;
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let path_string = input.trim();
    let path = Path::new(&path_string);
    if !path.exists() && !is_output {
        Err(CompressionError::FileNotFound)
    } else {
        Ok(path.to_path_buf())
    }
}
fn compress(path: &Path, output_path: &Path) -> Result<(), CompressionError> {
    let file = std::fs::File::open(path)?;
    let file_name = path.file_name().unwrap();
    let file_extension = path.extension().unwrap();
    let mut output = output_path.to_path_buf();
    output.push(file_name);
    output.set_extension(file_extension);
    output.add_extension("gz");
    println!("{:?}", output);
    let mut reader = BufReader::new(file);
    let output_file = OpenOptions::new()
        .truncate(true)
        .create(true)
        .write(true)
        .open(output)?;
    let writer = BufWriter::new(output_file);
    let mut encoder= GzEncoder::new(writer, Compression::default());
    io::copy(&mut reader, &mut encoder)?;
    encoder.finish()?;
    Ok(())
}

fn decompress(path: &Path, output_path: &Path) -> Result<(), CompressionError> {
    if path.extension().and_then(|s| s.to_str()) != Some("gz") {
       return  Err(CompressionError::InvalidExtension)
    }
    let file_name = path.file_name().unwrap();
    let mut output = output_path.to_path_buf();
    output.push(file_name);

    output.set_extension("");
    println!("{:?}", output);
    let file = std::fs::File::open(path)?;
    let  reader = BufReader::new(file);
    let output_file = OpenOptions::new()
        .truncate(true)
        .create(true)
        .write(true)
        .open(output)?;
    let mut writer = BufWriter::new(output_file);
    let mut decoder= GzDecoder::new(reader);
    io::copy(&mut decoder, &mut writer)?;

    Ok(())
}

