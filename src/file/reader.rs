/// A reader *reads* data into Storaged for processing. Because file data
use std::{
    fs::File,
    io::{Error, Read, Result},
};

use crate::utils::common::bytes_to_str;

/// Reader *reads* data into Storaged. Becase the file can be very
/// large, it does not make sense for `Reader` to load all of this
/// data into memory. Instead, `Reader` will load the file chunk
/// by chunk and delegate further processing.
pub struct Reader {
    file_name: String,
    chunk_size: u16,
    process_async: bool,
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

            if n == chunk_size {
                println!("({}b)\t{}", chunk_size, bytes_to_str(&buf)?);
            } else if n == 0 {
                // no more bytes to be produced
                break;
            } else {
                println!("({}b)\t{}", n, bytes_to_str(&buf[..n])?);
                break;
            }
        }

        Ok(bytes_read)
    }
}

pub struct ReaderBuilder {
    file_name: Option<String>,
    chunk_size: Option<u16>,
}

impl ReaderBuilder {
    pub fn new() -> Self {
        ReaderBuilder {
            file_name: None,
            chunk_size: None,
        }
    }

    pub fn file_name(mut self, file_name: String) -> Self {
        self.file_name = Some(file_name);
        self
    }

    pub fn chunk_size(mut self, chunk_size: u16) -> Self {
        self.chunk_size = Some(chunk_size);
        self
    }

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
                    "Chunk size must be between 16 to 2048 bytes inclusive",
                ));
            }
        }

        // Impossible to have a non-empty filename.
        // If a chunk size was not provided then default to 256 bytes.
        // By default, do not process async.
        Ok(Reader {
            file_name: self.file_name.unwrap(),
            chunk_size: self.chunk_size.unwrap_or(256),
            process_async: false,
        })
    }
}
