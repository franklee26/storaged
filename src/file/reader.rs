use std::{
    fs::File,
    io::{Error, Read, Result},
};

use crate::{set_some_builder_field, utils::common::bytes_to_str};

/// `ChunkReaderHandler` provides the call back that the reader will
/// execute during read operations.
pub trait ChunkReaderHandler {
    fn handle(&self, bytes: &[u8]) -> Result<()>;
}

struct NoOpHandler;

impl ChunkReaderHandler for NoOpHandler {
    fn handle(&self, _bytes: &[u8]) -> Result<()> {
        println!("No op!");
        Ok(())
    }
}

/// Reader *reads* data into Storaged. Becase the file can be very
/// large, it does not make sense for `Reader` to load all of this
/// data into memory. Instead, `Reader` will load the file chunk
/// by chunk and delegate further processing.
pub struct Reader {
    file_name: String,
    chunk_size: u16,
    process_async: bool,
    chunk_handler: Box<dyn ChunkReaderHandler>,
}

impl Reader {
    pub fn read(&mut self) -> Result<usize> {
        let chunk_size: usize = self.chunk_size.into();
        let mut file = File::open(&self.file_name)?;

        if let Ok(metadata) = file.metadata() {
            // TODO: Fine tune threshold file size that warrants async
            if metadata.len() > 2048 {
                self.process_async = true;
            }
        }

        let mut bytes_read = 0;
        let mut buf = vec![0u8; chunk_size];

        while let Ok(n) = file.read(&mut buf) {
            bytes_read += n;

            if n == 0 {
                // no more bytes to be produced
                break;
            }

            if n == chunk_size {
                println!("({}b)\t{}", chunk_size, bytes_to_str(&buf)?);
                self.chunk_handler.handle(&buf)?;
            } else {
                println!("({}b)\t{}", n, bytes_to_str(&buf[..n])?);
                self.chunk_handler.handle(&buf[..n])?;
                break;
            }
        }

        Ok(bytes_read)
    }
}

pub enum ReaderType {
    NoOp,
}

pub struct ReaderBuilder {
    file_name: Option<String>,
    chunk_size: Option<u16>,
    reader_type: Option<ReaderType>,
}

impl ReaderBuilder {
    pub fn new() -> Self {
        ReaderBuilder {
            file_name: None,
            chunk_size: None,
            reader_type: None,
        }
    }

    set_some_builder_field!(file_name, String);
    set_some_builder_field!(chunk_size, u16);
    set_some_builder_field!(reader_type, ReaderType);

    pub fn build(self) -> Result<Reader> {
        // Cannot have an empty file name
        if self.file_name.is_none() {
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "File name cannot be empty",
            ));
        }

        // Validate chunk size if it was provided
        if let Some(chunk_size) = self.chunk_size {
            if chunk_size < 16 || chunk_size > 1024 {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Chunk size must be between 16 to 1024 bytes inclusive",
                ));
            }
        }

        if self.reader_type.is_none() {
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "Reader type cannot be empty",
            ));
        }

        let handler: Box<dyn ChunkReaderHandler> = match self.reader_type.unwrap() {
            ReaderType::NoOp => Box::new(NoOpHandler),
        };

        // Impossible to have a non-empty filename.
        // If a chunk size was not provided then default to 256 bytes.
        // By default, do not process async.
        Ok(Reader {
            file_name: self.file_name.unwrap(),
            chunk_size: self.chunk_size.unwrap_or(256),
            process_async: false,
            chunk_handler: handler,
        })
    }
}
