#[cfg(feature = "smol_async")]
mod smol_async_tests {
    use std::io;
    use std::net::{TcpListener, TcpStream, UdpSocket};

    use futures_util::io::{AsyncReadExt, AsyncWriteExt};
    use futures_util::stream::StreamExt;
    use smol::{run, Async};

    use futures_util_io_peek::AsyncPeekExt;

    // ref https://github.com/stjepang/smol/blob/v0.1.18/tests/async_io.rs#L57-L77
    #[test]
    fn tcp_stream() -> io::Result<()> {
        run(async {
            let listener = Async::<TcpListener>::bind("127.0.0.1:0")?;
            let addr = listener.get_ref().local_addr()?;

            let mut stream_c = Async::<TcpStream>::connect(addr).await?;
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

    #[test]
    fn udp_socket() -> io::Result<()> {
        run(async {
            let socket1 = Async::<UdpSocket>::bind("127.0.0.1:0")?;
            let mut socket2 = Async::<UdpSocket>::bind("127.0.0.1:0")?;
            socket1.get_ref().connect(socket2.get_ref().local_addr()?)?;

            println!("socket1 {:?} socket2 {:?}", socket1, socket2);

            //
            let mut buf = vec![0; 5];

            socket1.send(vec![1, 2, 3].as_ref()).await?;

            let n = socket2.peek_async(&mut buf).await?;
            assert_eq!(buf, vec![1, 2, 3, 0, 0]);
            assert_eq!(n, 3);

            let n = socket2.recv(&mut buf).await?;
            assert_eq!(buf, vec![1, 2, 3, 0, 0]);
            assert_eq!(n, 3);

            Ok(())
        })
    }
}
