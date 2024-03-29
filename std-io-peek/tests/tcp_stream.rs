use std::{
    io::{Read as _, Write as _},
    net::{TcpListener, TcpStream},
};

use std_io_peek::Peek as _;

#[test]
fn sample() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let addr = listener.local_addr()?;

    let mut stream_c = TcpStream::connect(addr)?;
    let mut stream_s = listener
        .incoming()
        .next()
        .expect("Get next incoming failed")?;

    println!("addr:{addr:?}, stream_c:{stream_c:?} stream_s:{stream_s:?}");

    //
    let mut buf = vec![0; 5];

    stream_s.write_all(vec![1, 2, 3].as_ref())?;

    let n = stream_c.peek_sync(&mut buf)?;
    assert_eq!(buf, vec![1, 2, 3, 0, 0]);
    assert_eq!(n, 3);

    stream_s.write_all(vec![4].as_ref())?;

    let n = stream_c.peek_sync(&mut buf)?;
    assert_eq!(buf, vec![1, 2, 3, 4, 0]);
    assert_eq!(n, 4);

    let n = stream_c.read(&mut buf)?;
    assert_eq!(buf, vec![1, 2, 3, 4, 0]);
    assert_eq!(n, 4);

    Ok(())
}
