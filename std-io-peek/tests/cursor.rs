use std::io::{Cursor, Read as _};

use std_io_peek::Peek;

#[test]
fn sample() -> Result<(), Box<dyn std::error::Error>> {
    let mut cursor = Cursor::new(vec![1, 2, 3]);
    let mut buf = vec![0; 5];

    let n = cursor.peek_sync(&mut buf)?;
    assert_eq!(buf, vec![1, 2, 3, 0, 0]);
    assert_eq!(n, 3);

    cursor.get_mut().push(4);

    let n = cursor.peek_sync(&mut buf)?;
    assert_eq!(buf, vec![1, 2, 3, 4, 0]);
    assert_eq!(n, 4);

    let n = cursor.read(&mut buf)?;
    assert_eq!(buf, vec![1, 2, 3, 4, 0]);
    assert_eq!(n, 4);

    Ok(())
}
