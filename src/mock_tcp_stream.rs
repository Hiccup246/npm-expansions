use super::*;
use std::{cmp::min, io::Error};

pub struct MockTcpStream {
    pub read_data: Vec<u8>,
    pub write_data: Vec<u8>,
}

impl Read for MockTcpStream {
    fn read(self: &mut Self, buf: &mut [u8]) -> Result<usize, Error> {
        let size: usize = min(self.read_data.len(), buf.len());
        buf[..size].copy_from_slice(&self.read_data[..size]);
        Ok(size)
    }
}

impl Write for MockTcpStream {
    fn write(mut self: &mut Self, buf: &[u8]) -> Result<usize, Error> {
        self.write_data = Vec::from(buf);

        Ok(buf.len())
    }

    fn flush(self: &mut Self) -> Result<(), Error> {
        Ok(())
    }
}
