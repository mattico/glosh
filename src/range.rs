use std::default::Default;

/// Helper struct to generate Range header value
pub struct Range {
  offset: usize,
  pub step: usize
}
impl Range {
  /// Return next value and move the offset
  pub fn step(&mut self) -> String {
    let out = format!("{}-{}", self.offset, self.offset + self.step);
    self.offset = self.offset + self.step;
    out
  }

}

impl Default for Range {
  fn default() -> Range {
    Range {
      offset: 0,
      step: 100 * 1024 * 1024
    }
  }
}