use crate::constants::BYTE;

#[derive(PartialEq)]
pub enum CharByteSegment {
  Start,
  FirstContinuation,
  SecondContinuation,
  ThirdContinuation,
}

impl CharByteSegment {
  //   pub fn extract_bits(&self, byte: u8) -> u8 {
  //     let bits: u8 = if self == &CharByteSegment::Start {
  //       1
  //     } else {
  //       6
  //     };
  //     let offset = BYTE - (bits - 1);
  //     return (byte << offset) >> offset;
  //   }
  pub fn get_value(&self) -> (u8, u8) {
    match self {
      CharByteSegment::Start => (0b1111_0010, 1),
      _ => (0b1000_0000, 6),
    }
  }
}

impl Iterator for CharByteSegment {
  /// Returns the char byte mask, and the char bit space
  type Item = (u8, u8);

  fn next(&mut self) -> Option<Self::Item> {
    match self {
      CharByteSegment::Start => {
        *self = CharByteSegment::FirstContinuation;
        Some((0b1111_0010, 1))
      }
      CharByteSegment::FirstContinuation => {
        *self = CharByteSegment::SecondContinuation;
        Some((0b1000_0000, 6))
      }
      CharByteSegment::SecondContinuation => {
        *self = CharByteSegment::ThirdContinuation;
        Some((0b1000_0000, 6))
      }
      CharByteSegment::ThirdContinuation => {
        *self = CharByteSegment::Start;
        Some((0b1000_0000, 6))
      }
    }
  }
}
