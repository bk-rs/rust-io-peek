#![forbid(unsafe_code)]

//
mod lib_io;
pub use lib_io::AsyncPeek;

// ref https://github.com/rust-lang/futures-rs/blob/0.3.5/futures-util/src/io/mod.rs#L132-L382
pub trait AsyncPeekExt: AsyncPeek {
    fn peek_async<'a>(&'a mut self, buf: &'a mut [u8]) -> Peek<'a, Self>
    where
        Self: Unpin,
    {
        Peek::new(self, buf)
    }
}
impl<R: AsyncPeek + ?Sized> AsyncPeekExt for R {}

// ref https://github.com/rust-lang/futures-rs/blob/0.3.5/futures-util/src/io/read.rs
mod peek {
    use crate::AsyncPeek;

    use core::{
        future::Future,
        pin::Pin,
        task::{Context, Poll},
    };
    use std::io;

    pub struct Peek<'a, R: ?Sized> {
        reader: &'a mut R,
        buf: &'a mut [u8],
    }

    impl<R: ?Sized + Unpin> Unpin for Peek<'_, R> {}

    impl<'a, R: AsyncPeek + ?Sized + Unpin> Peek<'a, R> {
        pub(super) fn new(reader: &'a mut R, buf: &'a mut [u8]) -> Self {
            Peek { reader, buf }
        }
    }

    impl<R: AsyncPeek + ?Sized + Unpin> Future for Peek<'_, R> {
        type Output = io::Result<usize>;
        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            let this = &mut *self;
            Pin::new(&mut this.reader).poll_peek(cx, this.buf)
        }
    }
}
pub use peek::*;

// ref https://github.com/rust-lang/futures-rs/blob/0.3.5/futures-util/src/io/cursor.rs#L163-L185
mod cursor {
    use crate::AsyncPeek;

    use core::{
        pin::Pin,
        task::{Context, Poll},
    };
    use std::io;

    use futures_util::io::{AsyncBufRead as _, Cursor};

    impl<T: AsRef<[u8]> + Unpin> AsyncPeek for Cursor<T> {
        fn poll_peek(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut [u8],
        ) -> Poll<io::Result<usize>> {
            self.poll_fill_buf(cx).map(|r| match r {
                Ok(mut bytes) => {
                    let n = bytes.len();
                    io::copy(&mut bytes, &mut Box::new(buf))?;
                    Ok(n)
                }
                Err(e) => Err(e),
            })
        }
    }
}
