use crate::set_some_builder_field;
use std::{
    fs::File,
    io::{Error, Result, Write},
};

/// Writer *writes* data from storaged onto disk.
pub struct Writer {
    chunk_size: u16,
    process_async: bool,
    chunks_per_file: usize,
    file: File,
}

impl Writer {
    pub fn write(&mut self, bytes: &[u8]) -> Result<usize> {
        if bytes.len() > self.chunk_size.into() {
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "Provided number of bytes exceed maximum chunk length",
            ));
        }

        let bytes_written = self.file.write(bytes)?;
        Ok(bytes_written)
    }
}

pub struct WriterBuilder {
    file_name: Option<String>,
    chunk_size: Option<u16>,
    process_async: Option<bool>,
    chunks_per_file: Option<usize>,
}

impl WriterBuilder {
    pub fn new() -> Self {
        WriterBuilder {
            file_name: None,
            chunk_size: None,
            process_async: None,
            chunks_per_file: None,
        }
    }

    set_some_builder_field!(file_name, String);
    set_some_builder_field!(chunk_size, u16);
    set_some_builder_field!(process_async, bool);
    set_some_builder_field!(chunks_per_file, usize);

    #[inline]
    fn open_file(&self) -> Result<File> {
        let file = File::create(self.file_name.as_ref().unwrap())?;
        Ok(file)
    }

    pub fn build(self) -> Result<Writer> {
        // Cannot have an empty file name
        if self.file_name.is_none() {
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "File name cannot be empty",
            ));
        }

        // Chunk size must be provided
        match self.chunk_size {
            None => {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Chunk size cannot be empty",
                ))
            }
            Some(c_size) => {
                if c_size < 16 || c_size > 1024 {
                    return Err(Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Chunk size must be between 16 to 1024 bytes inclusive",
                    ));
                }
            }
        }

        if let Some(chunks_per_file) = self.chunks_per_file {
            if chunks_per_file <= 0 || chunks_per_file > 256 {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Num chunks per file must be more than zero and less than or equal to 256.",
                ));
            }
        }

        let file = self.open_file()?;

        Ok(Writer {
            chunk_size: self.chunk_size.unwrap(),
            process_async: self.process_async.unwrap_or(false),
            chunks_per_file: self.chunks_per_file.unwrap_or(64),
            file,
        })
    }
}
