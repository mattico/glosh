use std::sync::mpsc::{Receiver};
use std::io::{Read, Error};
pub struct ChannelReader {
  channel: Receiver<u8>
}
impl ChannelReader {
  pub fn new(recv: Receiver<u8>) -> ChannelReader {
    ChannelReader {
      channel: recv
    }
  }
}

impl Read for ChannelReader {
  fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
    let mut took: usize = 0;
    for (i, byte) in self.channel.iter().take(buf.len()).enumerate() {
        buf[i] = byte;
        took = i
    }
    Ok(took)
  }
}