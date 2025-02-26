use std::fmt;
use std::pin::Pin;

use futures::io::AsyncWrite;

use crate::io;
use crate::task::{Context, Poll};

/// Creates a writer that consumes and drops all data.
///
/// # Examples
///
/// ```rust
/// # #![feature(async_await)]
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::io;
/// use async_std::prelude::*;
///
/// let mut writer = io::sink();
/// writer.write(b"hello world").await?;
/// #
/// # Ok(()) }) }
/// ```
pub fn sink() -> Sink {
    Sink { _priv: () }
}

/// A writer that consumes and drops all data.
///
/// This writer is constructed by the [`sink`] function.
///
/// [`sink`]: fn.sink.html
pub struct Sink {
    _priv: (),
}

impl fmt::Debug for Sink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("Sink { .. }")
    }
}

impl AsyncWrite for Sink {
    #[inline]
    fn poll_write(
        self: Pin<&mut Self>,
        _: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Poll::Ready(Ok(buf.len()))
    }

    #[inline]
    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }

    #[inline]
    fn poll_close(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}
