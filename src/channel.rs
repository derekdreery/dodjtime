//! A wrapper around `futures_intrusive` that is specialised for my use.
use futures_intrusive::{
    buffer::ArrayBuf,
    channel::{ChannelStream, GenericChannel},
    NoopLock,
};

pub struct Channel<Cmd, const LEN: usize> {
    inner: GenericChannel<NoopLock, Cmd, ArrayBuf<Cmd, LEN>>,
}

impl<Cmd, const LEN: usize> Channel<Cmd, LEN> {
    pub fn new() -> Self {
        Channel {
            inner: GenericChannel::new(),
        }
    }

    pub fn stream(&self) -> ChannelStream<'_, NoopLock, Cmd, ArrayBuf<Cmd, LEN>> {
        self.inner.stream()
    }
}
