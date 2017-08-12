use rb::RbConsumer;
use std::io::{self, Read};
pub struct BufferedReader<T: RbConsumer<u8>> {
  channel: T
}
impl<T: RbConsumer<u8>> BufferedReader<T> {
  pub fn new(recv: T) -> BufferedReader<T> {
    BufferedReader {
      channel: recv
    }
  }
}

impl<T: RbConsumer<u8>> Read for BufferedReader<T> {
  fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    match self.channel.read_blocking(buf) {
      Some(len) => Ok(len),
      None => Err(io::ErrorKind::Other.into()),
    }
  }
}