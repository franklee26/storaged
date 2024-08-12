use std::io::Result;

/// `ChunkReaderHandler` provides the call back that the reader will
/// execute during read operations.
pub trait ChunkReader {
    fn handle(&self, bytes: &[u8]) -> Result<()>;
}

/// Dummy handler for filler purposes
pub struct NoOpHandler;

impl ChunkReader for NoOpHandler {
    fn handle(&self, _bytes: &[u8]) -> Result<()> {
        println!("No op!");
        Ok(())
    }
}
