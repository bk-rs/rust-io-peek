#[cfg(feature = "tokio_tcp_stream")]
mod tokio_tcp_stream_tests {
    use std::io;
    use std::net::Ipv4Addr;

    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::{TcpListener, TcpStream};
    use tokio::runtime::Runtime;
    use tokio::stream::StreamExt;

    use futures_util_io_peek::AsyncPeekExt;

    #[test]
    fn sample() -> io::Result<()> {
        Runtime::new()?.block_on(async {
            let mut listener = TcpListener::bind((Ipv4Addr::new(127, 0, 0, 1), 0)).await?;
            let addr = listener.local_addr()?;

            let mut stream_c = TcpStream::connect(addr).await?;
            let mut stream_s = listener
                .incoming()
                .next()
                .await
                .expect("Get next incoming failed")?;

            println!(
                "addr {:?}, stream_c {:?} stream_s {:?}",
                addr, stream_c, stream_s
            );

            //
            let mut buf = vec![0; 5];

            stream_s.write(vec![1, 2, 3].as_ref()).await?;

            let n = stream_c.peek_async(&mut buf).await?;
            assert_eq!(buf, vec![1, 2, 3, 0, 0]);
            assert_eq!(n, 3);

            stream_s.write(vec![4].as_ref()).await?;

            let n = stream_c.peek_async(&mut buf).await?;
            assert_eq!(buf, vec![1, 2, 3, 4, 0]);
            assert_eq!(n, 4);

            let n = stream_c.read(&mut buf).await?;
            assert_eq!(buf, vec![1, 2, 3, 4, 0]);
            assert_eq!(n, 4);

            Ok(())
        })
    }
}
