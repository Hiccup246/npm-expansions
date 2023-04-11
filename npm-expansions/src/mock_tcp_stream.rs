use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::net::SocketAddrV4;
use std::{
    cmp::min,
    io,
    io::{Error, Read, Write},
    net::TcpStream,
};

/// A mock of a TCP stream for testing purposes
pub struct MockTcpStream {
    /// Fake data which can be read from the mock tcp stream
    pub read_data: Vec<u8>,
    /// A place for fake data to be written to the mock tcp stream
    pub write_data: Vec<u8>,
}

impl Read for MockTcpStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
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

    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

/// Trait exposing TcpStream ip address functionality
pub trait TcpAddr {
    /// Returns the socket address of the remote peer of this TCP connection.
    fn peer_addr(&self) -> io::Result<SocketAddr>;
}

impl TcpAddr for MockTcpStream {
    fn peer_addr(&self) -> io::Result<SocketAddr> {
        Ok(SocketAddr::V4(SocketAddrV4::new(
            Ipv4Addr::new(127, 0, 0, 1),
            8080,
        )))
    }
}

impl TcpAddr for TcpStream {
    fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.peer_addr()
    }
}
