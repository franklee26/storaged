use std::io::{self, Error, Result, Write};

use file::reader::ReaderBuilder;

mod file;
mod utils;

const DEFAULT_CHUNK_SIZE_IN_BYTES: u16 = 256;

fn read_file_name() -> Result<String> {
    print!(">>> Input file name to read: ");
    io::stdout().flush()?;

    let mut file_name = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut file_name)?;

    file_name = file_name.trim().to_string();

    if file_name.is_empty() {
        return Err(Error::new(
            io::ErrorKind::InvalidInput,
            "File name cannot be empty",
        ));
    }

    Ok(file_name)
}

fn read_chunk_size() -> Result<u16> {
    let mut ans = DEFAULT_CHUNK_SIZE_IN_BYTES;
    print!(
        ">>> Input chunk size (defaults to {} bytes): ",
        DEFAULT_CHUNK_SIZE_IN_BYTES
    );
    io::stdout().flush()?;

    let mut chunk_size = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut chunk_size)?;

    chunk_size = chunk_size.trim().to_string();

    if let Ok(chunk_size_int) = chunk_size.parse::<u16>() {
        ans = chunk_size_int;
    }

    Ok(ans)
}

fn main() -> Result<()> {
    let file_name = read_file_name()?;
    println!("Reading file {}", file_name);

    let chunk_size = read_chunk_size()?;

    let mut reader = ReaderBuilder::new()
        .file_name(file_name.clone())
        .chunk_size(chunk_size)
        .build()?;

    let bytes_read = reader.read()?;
    println!("Read {} bytes", bytes_read);

    Ok(())
}
