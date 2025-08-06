use crate::{CharByteSegment, constants::BYTE};

pub struct UTF8Char {
  char: [u8; 4],
  cur_bits_left: u8,
  cur_seg: CharByteSegment,
}

impl UTF8Char {
  /// Pulls the requested number of bits from the 4-byte char then returns the data and the quantity of the pulled bits.
  ///
  /// The output depends on the amount of bits left in the char, so it's
  /// likely for data to get less bits than it requested for
  ///
  /// # Returns
  /// (data, bit_quantity)
  pub fn get_bits(&mut self, mut requested_bits: u8) -> (u8, u8) {
    let (mask, mask_size) = self
      .cur_seg
      .create_data_mask(self.cur_bits_left, requested_bits % BYTE);

    requested_bits -= mask_size;
    self.cur_bits_left -= mask_size;

    let remainder_bits = requested_bits % BYTE;
    let byte = self.char[self.cur_seg.idx()];

    if self.cur_bits_left == 0 && self.cur_seg != CharByteSegment::ThirdContinuation {
      self.cur_seg.next();
      self.cur_bits_left = self.cur_seg.get_size();
    }

    return ((byte & mask) << remainder_bits, requested_bits);
  }

  /// Checks whether all of the bits on this char have already been pulled
  pub fn is_end(&self) -> bool {
    self.cur_bits_left == 0 && self.cur_seg == CharByteSegment::ThirdContinuation
  }
}

impl TryFrom<&[u8]> for UTF8Char {
  type Error = &'static str;

  fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
    if value.len() != 4 {
      return Err("UTF8Char: Received value with a length of either less or greater than 4");
    }

    let cur_seg = CharByteSegment::Start;
    let cur_bits_left = cur_seg.get_size();

    Ok(Self {
      // SAFETY: the length of the vector was verified to be the correct length beforehand
      char: unsafe { value.try_into().unwrap_unchecked() },
      cur_bits_left,
      cur_seg,
    })
  }
}
