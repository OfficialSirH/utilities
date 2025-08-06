use std::{iter::Peekable, vec::IntoIter};

use crate::{CharByteSegment, DataKind, Schema, UTF8Char};

pub struct DataBuffer {
  schema: Schema,
  buffer: Peekable<IntoIter<UTF8Char>>,
}

impl DataBuffer {
  pub fn new(schema: Schema, buffer: Vec<u8>) -> Self {
    if buffer.len() % 4 != 0 {
      panic!("I ain't got time to properly handle this for right now")
    }

    let mut utf8_buffer = Vec::new();
    for chunk in buffer.utf8_chunks() {
      let char_bytes = chunk.valid().as_bytes();
      let utf8_char = UTF8Char::try_from(char_bytes).unwrap();
      utf8_buffer.push(utf8_char);
    }
    let buffer = utf8_buffer.into_iter().peekable();

    Self { schema, buffer }
  }
}

impl Iterator for DataBuffer {
  type Item = u8;

  fn next(&mut self) -> Option<Self::Item> {
    // TODO: Implemenet the following:
    // - UTF8Char bit drain function
    // - Schema consume-to-byte function
    // The potential code below?
    let mut char = self.buffer.peek_mut()?;

    let mut is_complete = self.schema.consume_for_byte(char);
    while !is_complete {
      self.buffer.next();
      char = self.buffer.peek_mut()?;
      is_complete = self.schema.consume_for_byte(char);
    }

    return Some(self.schema.output());
  }
}
