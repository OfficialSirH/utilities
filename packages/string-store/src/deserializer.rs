use std::{ops::IndexMut, slice::Iter};

use crate::{
  DataKind,
  char_byte_segment::CharByteSegment,
  constants::{BYTE, DYNAMIC_DATA_INDICATOR, MAX_CUSTOM_ID_SIZE, MAX_SAFE_DYNAMIC_BYTE_LENGTH},
  data_requirement::get_data_requirement,
  parse_schema,
};

// TODO: change up this whole thing, it's not gonna work in its current state.
// schema format:
// enum DataKind {
//  Nullable(DataKind), (0)
//  Normal, (1)
//  Dynamic (2)
// }
// schema entries will be prefixed with the above DataKind values
pub struct Deserializer {
  schema: Vec<DataKind>,
  offset: u8,
  current_char_segment: CharByteSegment,
}

impl Deserializer {
  pub fn new(unparsed_schema: Vec<u8>) -> Result<Self, &'static str> {
    let schema = parse_schema(&unparsed_schema)?;
    Ok(Self {
      schema,
      offset: 0,
      current_char_segment: CharByteSegment::Start,
    })
  }

  // pub fn deserialize(&mut self, buffer: Vec<u8>) -> Result<Vec<u8>, &'static str> {
  //   // ensures bare minimum requirements met without the need for the values
  //   let _ = get_data_requirement(&self.schema, &buffer)?;

  //   let required_bytes = self
  //     .schema
  //     .iter()
  //     .cloned()
  //     .fold(0, |acc, count| acc + (count != 0) as usize);

  //   let deserialized_buffer = self.write_deserialized_buffer(buffer, required_bytes)?;

  //   Ok(deserialized_buffer)
  // }

  // fn write_deserialized_buffer(
  //   &mut self,
  //   buffer: Vec<u8>,
  //   required_bytes: usize,
  // ) -> Result<Vec<u8>, &'static str> {
  //   let mut deserialized_buffer = vec![0; required_bytes];
  //   let mut deserialized_buffer_iterator = deserialized_buffer.iter_mut();
  //   let mut data_segment = deserialized_buffer_iterator.next().unwrap();
  //   let mut schema_iterator = self.schema.iter();
  //   let mut schema_segment = schema_iterator.next().unwrap();
  //   let mut shift_offset = schema_segment.saturating_sub(1);
  //   let mut expected_char_segment = CharByteSegment::Start;
  //   for char_byte in buffer {
  //     let (char_mask, char_bit_space) = expected_char_segment.next().unwrap();

  //     if char_byte & char_mask != char_mask {
  //       return Err("Buffer has invalid data");
  //     }

  //     let mut extract_n_iterate = |bit_count: u8| {
  //       for bit_index in (0..bit_count).rev() {
  //         *data_segment |= ((char_byte >> bit_index) & 1) << shift_offset;
  //         if shift_offset == 0 {
  //           // Dynamic values
  //           // if *schema_segment == DYNAMIC_DATA_INDICATOR {
  //           //   self.schema = vec![vec![8; *data_segment as usize], self.schema.clone()].concat();
  //           //   schema_iterator = self.schema.iter();
  //           // }
  //           // Nullable values
  //           if *schema_segment == 0 {
  //             if *data_segment == 0 {
  //               println!("the sacred NULL");
  //               let nullable_bytes = schema_iterator.next().unwrap();
  //               for _ in 0..*nullable_bytes {
  //                 schema_iterator.next();
  //                 deserialized_buffer_iterator.next();
  //               }
  //             } else {
  //               schema_iterator.next();
  //             }
  //           }
  //           match deserialized_buffer_iterator.next() {
  //             Some(next_segment) => data_segment = next_segment,
  //             None => return false,
  //           };
  //           schema_segment = schema_iterator.next().unwrap();
  //           if *schema_segment == DYNAMIC_DATA_INDICATOR {
  //             // we need the length prefix, so we're manually setting the shift_offset
  //             shift_offset = 8;
  //             continue;
  //           }
  //           shift_offset = schema_segment.saturating_sub(1);
  //           continue;
  //         }
  //         shift_offset -= 1;
  //       }
  //       true
  //     };
  //     if !extract_n_iterate(char_bit_space) {
  //       break;
  //     }
  //   }
  //   Ok(deserialized_buffer)
  // }

  // pub fn rewrite_deserialize(&mut self, buffer: Vec<u8>) -> Result<Vec<u8>, &'static str> {
  //   // ensures bare minimum requirements met without the need for the values
  //   // let _ = get_data_requirement(&self.schema, &buffer)?;

  //   let deserialized_buffer = self.rewrite_write_deserialized_buffer(buffer)?;

  //   Ok(deserialized_buffer)
  // }

  // fn rewrite_write_deserialized_buffer(
  //   &mut self,
  //   buffer: Vec<u8>,
  // ) -> Result<Vec<u8>, &'static str> {
  //   let mut deserialized_buffer: Vec<u8> = Vec::with_capacity(MAX_CUSTOM_ID_SIZE);
  //   let deserialized_buffer_slice = deserialized_buffer.as_mut_slice();
  //   let mut buffer_index: usize = 0;
  //   let parsed_schema = parse_schema(&self.schema)?;
  //   let mut schema_iterator = parsed_schema.iter();
  //   let mut schema_segment = schema_iterator.next().unwrap();
  //   let (mut bits_space, mut shift_offset) = schema_segment.get_bits_n_offset();

  //   let mut expected_char_segment = CharByteSegment::Start;
  //   for char_byte in buffer {
  //     let (char_mask, char_bit_space) = expected_char_segment.next().unwrap();

  //     if char_byte & char_mask != char_mask {
  //       return Err("Buffer has invalid data");
  //     }

  //     let mut extract_n_iterate = |bit_count: u8| {
  //       for bit_index in (0..bit_count).rev() {
  //         *deserialized_buffer_slice.index_mut(buffer_index) |=
  //           ((char_byte >> bit_index) & 1) << shift_offset;
  //         if shift_offset > 0 {
  //           bits_space -= 1;
  //           shift_offset -= 1;
  //           continue;
  //         }

  //         if bits_space > 0 {
  //           shift_offset = bits_space.min(BYTE);
  //           buffer_index += 1;
  //           if buffer_index == MAX_CUSTOM_ID_SIZE {
  //             return false;
  //           }
  //           continue;
  //         }

  //         match *schema_segment {
  //           DataKind::Nullable => {
  //             if *deserialized_buffer_slice.index_mut(buffer_index) == 0 {
  //               schema_iterator.next();
  //             }
  //           }
  //           DataKind::Normal(bits) => {
  //             bits_space = bits;
  //             shift_offset = bits.min(8);
  //           }
  //           DataKind::Dynamic => {
  //             // OH FOR FUCKS SAKE, I NEED TO REWRITE EVERYTHING AT THIS POINT
  //             // awaiting_dynamic_length = true;
  //             bits_space = 8;
  //             shift_offset = 8;
  //           }
  //           DataKind::Fixed(bytes) => {
  //             bits_space = bytes * BYTE;
  //             shift_offset = BYTE;
  //           }
  //         }
  //         buffer_index += 1;
  //         if buffer_index == MAX_CUSTOM_ID_SIZE {
  //           return false;
  //         }
  //         schema_segment = schema_iterator.next().unwrap();
  //       }
  //       true
  //     };
  //     if !extract_n_iterate(char_bit_space) {
  //       break;
  //     }
  //   }
  //   Ok(deserialized_buffer)
  // }

  pub fn deserialize(&self, buffer: Vec<u8>) -> Result<Vec<u8>, &'static str> {
    todo!()
  }

  // pub fn update_buffer(&mut self, byte: &mut u8, bit_space: &mut u8, char: u8) {
  //   // bit_space - leftover space within the whole value's data
  //   // offset - offset of the char
  // }
}

#[cfg(test)]
mod test {
  #[test]
  fn new_deserialize() {
    let serialized_buffer = vec![1111_0010];
  }
}

// deserialize(buffer)
// (ensure buffer is divisible by 4 due to serialization/deserialization design)
// schema will have methods per DataKind:
// - handle
// encapsulate byte inside of a CharByteSegment variant
// CharByteSegment will have `update_buffer` method with params:
// &mut byte: u8
// &mut carried_bits: u8
// returns (emptied_char, filled_byte)
// return (bool, bool)
