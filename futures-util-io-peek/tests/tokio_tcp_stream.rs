#![cfg(feature = "tokio_tcp_stream")]

use std::net::Ipv4Addr;

use tokio::{
    io::{AsyncReadExt as _, AsyncWriteExt as _},
    net::{TcpListener, TcpStream},
};

use futures_util_io_peek::AsyncPeekExt;

#[tokio::test]
async fn tcp_stream() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind((Ipv4Addr::new(127, 0, 0, 1), 0)).await?;
    let addr = listener.local_addr()?;

    let mut stream_c = TcpStream::connect(addr).await?;
    let (mut stream_s, _) = listener.accept().await?;

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
}
