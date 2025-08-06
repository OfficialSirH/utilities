use crate::constants::BYTE;

#[derive(PartialEq)]
pub enum CharByteSegment {
  Start,
  FirstContinuation,
  SecondContinuation,
  ThirdContinuation,
}

impl CharByteSegment {
  pub fn create_data_mask(&self, mask_size: u8, requested_bits: u8) -> (u8, u8) {
    println!("DATA BITS LEFT: {requested_bits}");
    let (_, mask, _) = self.get_value();

    let bits_taken = (requested_bits == 0)
      .then(|| mask_size)
      .unwrap_or(requested_bits.min(mask_size));
    let mask_subtraction = 2_u8.pow(bits_taken as u32) - 1;

    return (mask - mask_subtraction, bits_taken);
  }

  /// Returns the char byte mask, data mask, and the bit space
  ///
  /// `(byte_mask, data_mask, bit_space)`
  pub fn get_value(&self) -> (u8, u8, u8) {
    match self {
      CharByteSegment::Start => (0b1111_0010, 0b0000_0001, 1),
      _ => (0b1000_0000, 0b0011_1111, 6),
    }
  }

  pub fn get_size(&self) -> u8 {
    self.get_value().2
  }

  pub fn idx(&self) -> usize {
    match self {
      CharByteSegment::Start => 0,
      CharByteSegment::FirstContinuation => 1,
      CharByteSegment::SecondContinuation => 2,
      CharByteSegment::ThirdContinuation => 3,
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
