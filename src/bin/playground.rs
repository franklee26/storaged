use std::{
    cmp,
    io::{self, Error, Result, Write},
};

use storaged::file::{
    reader::{ReaderBuilder, ReaderType},
    writer::WriterBuilder,
};

const DEFAULT_CHUNK_SIZE_IN_BYTES: u16 = 256;
const DEFAULT_NUMBER_OF_BYTES_TO_WRITE: usize = 1024;

enum Command {
    Read,
    ProcessToBlockFile,
    Write,
}

fn read_file_name(command: Command) -> Result<String> {
    match command {
        Command::Read => print!(">>> Input file name to read: "),
        Command::ProcessToBlockFile => {
            print!(">>> Input file name to process into block files: ")
        }
        Command::Write => {
            print!(">>> Input file name to write to: ")
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

fn read_number_of_bytes_to_write() -> Result<usize> {
    let mut ans = DEFAULT_NUMBER_OF_BYTES_TO_WRITE;
    print!(
        ">>> Input number of bytes to write (defaults to {} bytes): ",
        DEFAULT_NUMBER_OF_BYTES_TO_WRITE
    );
    io::stdout().flush()?;

    let mut bytes_to_write = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut bytes_to_write)?;

    bytes_to_write = bytes_to_write.trim().to_string();

    if let Ok(bytes_to_write_int) = bytes_to_write.parse::<usize>() {
        ans = bytes_to_write_int;
    }

    Ok(ans)
}

fn read_io_command() -> Result<String> {
    print!(">>> Read (r), process to block file (p), write file (w), exit (e): ");
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

fn write_random_bytes_command_handler() -> Result<()> {
    let file_name = read_file_name(Command::Write)?;
    println!("Writing to file {}", file_name);

    let chunk_size = read_chunk_size()?;

    let number_bytes_to_write = read_number_of_bytes_to_write()?;

    let mut writer = WriterBuilder::new()
        .file_name(file_name.clone())
        .chunk_size(chunk_size)
        .build()?;

    // Start off with ascii char for 'A'
    let mut iteration_num = 0x41;
    let mut total_bytes_written: usize = 0;

    while total_bytes_written < number_bytes_to_write.into() {
        let remaining_num_bytes: usize = number_bytes_to_write - total_bytes_written;
        let c_size = cmp::min(remaining_num_bytes, chunk_size.into());

        let data = vec![iteration_num; c_size];
        // rand::thread_rng().fill_bytes(&mut data);

        let bytes_written = writer.write(&data)?;
        total_bytes_written += bytes_written;

        iteration_num += 1;
        if iteration_num >= u8::MAX {
            iteration_num = 0x41;
        }
    }

    println!("Wrote {} bytes", total_bytes_written);

    Ok(())
}

fn main() -> Result<()> {
    loop {
        match read_io_command() {
            Ok(command) => {
                let res = match command.to_lowercase().as_str() {
                    "r" => read_command_handler(),
                    "p" => process_command_handler(16),
                    "w" => write_random_bytes_command_handler(),
                    "e" => return Ok(()),
                    _ => {
                        eprintln!("Usage: `r` to read, `p` to process to block file, `w` to write to file, `e` to exit");
                        Ok(())
                    }
                };

                if let Err(e) = res {
                    eprintln!("Encountered error: {}", e);
                }
            }
            Err(e) => eprintln!("Encountered error while reading comamnd: {e}"),
        }
    }
}
