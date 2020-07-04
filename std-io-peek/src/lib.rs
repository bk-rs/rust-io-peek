#![forbid(unsafe_code)]

use std::io;

pub trait Peek {
    fn peek_sync(&mut self, buf: &mut [u8]) -> io::Result<usize>;
}

mod tcp_stream {
    use super::*;

    use std::net::TcpStream;

    impl Peek for TcpStream {
        fn peek_sync(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            self.peek(buf)
        }
    }
}

mod cursor {
    use super::*;

    use std::io::Cursor;
    use std::io::{BufRead, Read};

    impl<T> Peek for Cursor<T>
    where
        T: AsRef<[u8]>,
    {
        fn peek_sync(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            let n = Read::read(&mut self.fill_buf()?, buf)?;
            Ok(n)
        }
    }
}
