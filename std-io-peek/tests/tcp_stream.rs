use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};

use std_io_peek::Peek;

#[test]
fn sample() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let addr = listener.local_addr()?;

    let mut stream_c = TcpStream::connect(addr)?;
    let mut stream_s = listener
        .incoming()
        .next()
        .expect("Get next incoming failed")?;

    println!(
        "addr {:?}, stream_c {:?} stream_s {:?}",
        addr, stream_c, stream_s
    );

    //
    let mut buf = vec![0; 5];

    stream_s.write(vec![1, 2, 3].as_ref())?;

    let n = stream_c.peek_sync(&mut buf)?;
    assert_eq!(buf, vec![1, 2, 3, 0, 0]);
    assert_eq!(n, 3);

    stream_s.write(vec![4].as_ref())?;

    let n = stream_c.peek_sync(&mut buf)?;
    assert_eq!(buf, vec![1, 2, 3, 4, 0]);
    assert_eq!(n, 4);

    let n = stream_c.read(&mut buf)?;
    assert_eq!(buf, vec![1, 2, 3, 4, 0]);
    assert_eq!(n, 4);

    Ok(())
}
