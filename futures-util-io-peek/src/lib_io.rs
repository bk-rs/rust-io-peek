use std::future::Future;
use std::io;
use std::ops::DerefMut;
use std::pin::Pin;
use std::task::{Context, Poll};

// ref https://github.com/rust-lang/futures-rs/blob/0.3.5/futures-io/src/lib.rs#L50-L129
pub trait AsyncPeek {
    fn poll_peek(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>>;
}

// ref https://github.com/rust-lang/futures-rs/blob/0.3.5/futures-io/src/lib.rs#L321-L396
macro_rules! deref_async_peek {
    () => {
        fn poll_peek(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut [u8],
        ) -> Poll<io::Result<usize>> {
            Pin::new(&mut **self).poll_peek(cx, buf)
        }
    };
}

impl<T: ?Sized + AsyncPeek + Unpin> AsyncPeek for Box<T> {
    deref_async_peek!();
}

impl<T: ?Sized + AsyncPeek + Unpin> AsyncPeek for &mut T {
    deref_async_peek!();
}

impl<P> AsyncPeek for Pin<P>
where
    P: DerefMut + Unpin,
    P::Target: AsyncPeek,
{
    fn poll_peek(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        self.get_mut().as_mut().poll_peek(cx, buf)
    }
}

// ref https://docs.rs/async-io
#[cfg(feature = "async_io_async")]
mod async_io_async {
    use super::*;

    use std::net::{TcpStream, UdpSocket};

    use async_io::Async;

    // ref https://docs.rs/async-io/0.1.1/async_io/struct.Async.html#method.peek
    impl AsyncPeek for Async<TcpStream> {
        fn poll_peek(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut [u8],
        ) -> Poll<io::Result<usize>> {
            poll_once(cx, self.peek(buf))
        }
    }

    // ref https://docs.rs/async-io/0.1.1/async_io/struct.Async.html#method.peek-1
    impl AsyncPeek for Async<UdpSocket> {
        fn poll_peek(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut [u8],
        ) -> Poll<io::Result<usize>> {
            poll_once(cx, self.peek(buf))
        }
    }
}

// ref https://docs.rs/smol
#[cfg(feature = "smol_async")]
mod smol_async {
    use super::*;

    use std::net::{TcpStream, UdpSocket};

    use smol::Async;

    // ref https://docs.rs/smol/0.1.18/smol/struct.Async.html#method.peek
    impl AsyncPeek for Async<TcpStream> {
        fn poll_peek(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut [u8],
        ) -> Poll<io::Result<usize>> {
            poll_once(cx, self.peek(buf))
        }
    }

    // ref https://docs.rs/smol/0.1.18/smol/struct.Async.html#method.peek-1
    impl AsyncPeek for Async<UdpSocket> {
        fn poll_peek(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut [u8],
        ) -> Poll<io::Result<usize>> {
            poll_once(cx, self.peek(buf))
        }
    }
}

// ref https://docs.rs/async-std
#[cfg(feature = "async_std_tcp_stream")]
mod async_std_tcp_stream {
    use super::*;

    use async_std::net::TcpStream;

    // ref https://docs.rs/async-std/1.6.2/async_std/net/struct.TcpStream.html#method.peek
    impl AsyncPeek for TcpStream {
        fn poll_peek(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut [u8],
        ) -> Poll<io::Result<usize>> {
            poll_once(cx, self.peek(buf))
        }
    }
}

// ref https://docs.rs/tokio
#[cfg(feature = "tokio_tcp_stream")]
mod tokio_tcp_stream {
    use super::*;

    use tokio::net::TcpStream;

    // https://docs.rs/tokio/0.2.21/tokio/net/struct.TcpStream.html#method.peek
    impl AsyncPeek for TcpStream {
        fn poll_peek(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut [u8],
        ) -> Poll<io::Result<usize>> {
            poll_once(cx, self.get_mut().peek(buf))
        }
    }
}

// ref https://github.com/stjepang/async-io/blob/v0.1.1/src/lib.rs#L1249
#[cfg(any(
    feature = "async_io_async",
    feature = "smol_async",
    feature = "async_std_tcp_stream",
    feature = "tokio_tcp_stream"
))]
fn poll_once<T>(cx: &mut Context<'_>, fut: impl Future<Output = T>) -> Poll<T> {
    futures_util::pin_mut!(fut);
    fut.poll(cx)
}
