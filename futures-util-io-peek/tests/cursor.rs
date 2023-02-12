use std::io;

use futures_executor::block_on;
use futures_util::io::{AsyncReadExt as _, Cursor};

use futures_util_io_peek::AsyncPeekExt as _;

#[test]
fn sample() -> io::Result<()> {
    block_on(async {
        let mut cursor = Cursor::new(vec![1, 2, 3]);
        let mut buf = vec![0; 5];

        let n = cursor.peek_async(&mut buf).await?;
        assert_eq!(buf, vec![1, 2, 3, 0, 0]);
        assert_eq!(n, 3);

        cursor.get_mut().push(4);

        let n = cursor.peek_async(&mut buf).await?;
        assert_eq!(buf, vec![1, 2, 3, 4, 0]);
        assert_eq!(n, 4);

        let n = cursor.read(&mut buf).await?;
        assert_eq!(buf, vec![1, 2, 3, 4, 0]);
        assert_eq!(n, 4);

        Ok(())
    })
}
