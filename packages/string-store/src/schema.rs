use std::{iter::Peekable, vec::IntoIter};

use crate::{DataKind, constants::BYTE};

pub struct Schema {
  data_kinds: Peekable<IntoIter<DataKind>>,
  output: u8,
  cur_needed_bits: u8,
}

impl Schema {
  /// Constructs Schema
  ///
  /// # Panic
  /// If you supply a zero-lengthed vector of DataKind, it will panic
  pub fn new(data_kinds: Vec<DataKind>) -> Self {
    if data_kinds.len() == 0 {
      panic!("Schema struct received an empty vector of data kinds")
    }

    let mut data_kinds = data_kinds.into_iter().peekable();

    let kind = data_kinds.peek().unwrap();
    let (cur_needed_bits, _) = kind.get_bits_n_offset();

    Self {
      data_kinds,
      output: 0,
      cur_needed_bits,
    }
  }

  pub fn consume_for_byte(&mut self, char: &mut crate::UTF8Char) -> bool {
    // retrieve bits from char
    // use the received mask to AND the output
    // reduce the current bits needed
    // return true ONLY if the current bits needed is 0

    let (data, bit_quantity) = char.get_bits(self.cur_needed_bits);

    self.cur_needed_bits -= bit_quantity;

    self.output &= data;

    return self.cur_needed_bits % BYTE == 0;
  }

  pub fn output(&mut self) -> u8 {
    if self.cur_needed_bits == 0 {
      let kind = self.data_kinds.next().unwrap();
      if kind == DataKind::Nullable && self.output == 0 {
        // Consume whatever Kind was preceding Nullable
        self.data_kinds.next();
      }

      if kind == DataKind::Dynamic {
        self.cur_needed_bits = self.output;
      } else if let Some(kind) = self.data_kinds.peek() {
        self.cur_needed_bits = kind.get_size();
      }
    }

    let output = self.output;
    self.output = 0;

    return output;
  }
}
