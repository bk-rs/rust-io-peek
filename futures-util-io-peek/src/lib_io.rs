use core::{
    ops::DerefMut,
    pin::Pin,
    task::{Context, Poll},
};
use std::io;

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

    use core::future::Future as _;
    use std::net::{TcpStream, UdpSocket};

    use async_io::Async;

    // ref https://docs.rs/async-io/1.12.0/async_io/struct.Async.html#method.peek
    impl AsyncPeek for Async<TcpStream> {
        fn poll_peek(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut [u8],
        ) -> Poll<io::Result<usize>> {
            let fut = self.peek(buf);
            futures_util::pin_mut!(fut);
            fut.poll(cx)
        }
    }

    // ref https://docs.rs/async-io/1.12.0/async_io/struct.Async.html#method.peek-1
    impl AsyncPeek for Async<UdpSocket> {
        fn poll_peek(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut [u8],
        ) -> Poll<io::Result<usize>> {
            let fut = self.peek(buf);
            futures_util::pin_mut!(fut);
            fut.poll(cx)
        }
    }
}

// ref https://docs.rs/tokio
#[cfg(feature = "tokio_tcp_stream")]
mod tokio_tcp_stream {
    use super::*;

    use core::future::Future as _;

    use tokio::net::TcpStream;

    // https://docs.rs/tokio/1.25.0/tokio/net/struct.TcpStream.html#method.peek
    impl AsyncPeek for TcpStream {
        fn poll_peek(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut [u8],
        ) -> Poll<io::Result<usize>> {
            let fut = self.get_mut().peek(buf);
            futures_util::pin_mut!(fut);
            fut.poll(cx)
        }
    }
}
