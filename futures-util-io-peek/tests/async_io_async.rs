#![cfg(feature = "async_io_async")]

use std::net::{TcpListener, TcpStream, UdpSocket};

use async_io::Async;
use futures_executor::block_on;
use futures_util::{
    io::{AsyncReadExt as _, AsyncWriteExt as _},
    pin_mut,
    stream::StreamExt as _,
};

use futures_util_io_peek::AsyncPeekExt;

// ref https://github.com/smol-rs/async-io/blob/master/tests/async.rs#L64
#[test]
fn tcp_stream() -> Result<(), Box<dyn std::error::Error>> {
    block_on(async {
        let listener = Async::<TcpListener>::bind(([127, 0, 0, 1], 0))?;
        let addr = listener.get_ref().local_addr()?;

        let mut stream_c = Async::<TcpStream>::connect(addr).await?;
        let incoming = listener.incoming();
        pin_mut!(incoming);
        let mut stream_s = incoming.next().await.ok_or("incoming.next none")??;

        println!("addr:{addr:?}, stream_c:{stream_c:?} stream_s:{stream_s:?}");

        //
        let mut buf = vec![0; 5];

        stream_s.write_all(vec![1, 2, 3].as_ref()).await?;

        let n = stream_c.peek_async(&mut buf).await?;
        assert_eq!(buf, vec![1, 2, 3, 0, 0]);
        assert_eq!(n, 3);

        stream_s.write_all(vec![4].as_ref()).await?;

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
fn udp_socket() -> Result<(), Box<dyn std::error::Error>> {
    block_on(async {
        let socket1 = Async::<UdpSocket>::bind(([127, 0, 0, 1], 0))?;
        let mut socket2 = Async::<UdpSocket>::bind(([127, 0, 0, 1], 0))?;
        socket1.get_ref().connect(socket2.get_ref().local_addr()?)?;

        println!("socket1:{socket1:?} socket2:{socket2:?}");

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
