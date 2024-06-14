use std::io::{self, Error, Result, Write};

use file::reader::{ReaderBuilder, ReaderType};

mod blocks;
mod file;
mod utils;

const DEFAULT_CHUNK_SIZE_IN_BYTES: u16 = 256;

enum Command {
    Read,
    ProcessToBlockFile,
}

fn read_file_name(command: Command) -> Result<String> {
    match command {
        Command::Read => print!(">>> Input file name to read: "),
        Command::ProcessToBlockFile => {
            print!(">>> Input file name to process into block files: ")
        }
    }

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

fn read_io_command() -> Result<String> {
    print!(">>> Read (r), process to block file (p), exit (e)? ");
    io::stdout().flush()?;

    let mut command = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut command)?;

    command = command.trim().to_string();

    Ok(command)
}

fn read_command_handler() -> Result<()> {
    let file_name = read_file_name(Command::Read)?;
    println!("Reading file {}", file_name);

    let chunk_size = read_chunk_size()?;

    let mut reader = ReaderBuilder::new()
        .reader_type(ReaderType::NoOp)
        .file_name(file_name.clone())
        .chunk_size(chunk_size)
        .build()?;

    let bytes_read = reader.read()?;
    println!("Read {} bytes", bytes_read);

    Ok(())
}

fn process_command_handler(chunk_size: u16) -> Result<()> {
    let file_name = read_file_name(Command::ProcessToBlockFile)?;
    println!("Processing {}", file_name);

    let mut reader = ReaderBuilder::new()
        .reader_type(ReaderType::NoOp)
        .file_name(file_name.clone())
        .chunk_size(chunk_size)
        .build()?;

    let bytes_read = reader.read()?;
    println!("Processed {} bytes", bytes_read);

    Ok(())
}

fn main() -> Result<()> {
    loop {
        if let Ok(command) = read_io_command() {
            let res = match command.to_lowercase().as_str() {
                "r" => read_command_handler(),
                "p" => process_command_handler(16),
                "e" => return Ok(()),
                _ => {
                    eprintln!("Usage: `r` to read, `p` to process to block file, `e` to exit");
                    Ok(())
                }
            };

            if let Err(e) = res {
                eprintln!("Encountered error: {}", e);
            }
        }
    }
}
