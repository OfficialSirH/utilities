use std::{ops::IndexMut, slice::Iter};

use crate::{
  DataBuffer, DataKind,
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
      schema: todo!(),
      offset: 0,
      current_char_segment: CharByteSegment::Start,
    })
  }

  // pub fn deserialize(&mut self, buffer: Vec<u8>) -> Result<Vec<u8>, &'static str> {
  //   // ensures bare minimum requirements met without the need for the values
  //   // let _ = get_data_requirement(&self.schema, &buffer)?;

  //   // let required_bytes = self
  //   //   .schema
  //   //   .iter()
  //   //   .cloned()
  //   //   .fold(0, |acc, count| acc + (count != 0) as usize);

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

  pub fn deserialize(&mut self, buffer: Vec<u8>) -> Result<Vec<u8>, &'static str> {
    // ensures bare minimum requirements met without the need for the values
    // let _ = get_data_requirement(&self.schema, &buffer)?;

    let deserialized_buffer = self.rewrite_write_deserialized_buffer(buffer)?;

    Ok(deserialized_buffer)
  }

  fn rewrite_write_deserialized_buffer(
    &mut self,
    buffer: Vec<u8>,
  ) -> Result<Vec<u8>, &'static str> {
    let mut deserialized_buffer: Vec<u8> = Vec::with_capacity(MAX_CUSTOM_ID_SIZE);
    let uninit_buffer = deserialized_buffer.spare_capacity_mut();
    // let deserialized_buffer_slice = deserialized_buffer.as_mut_slice();
    let mut buffer_index: usize = 0;
    let mut schema_iterator = self.schema.iter();
    let mut schema_segment = schema_iterator.next().unwrap();
    let (mut bits_space, mut shift_offset) = schema_segment.get_bits_n_offset();

    let mut expected_char_segment = CharByteSegment::Start;
    for char_byte in buffer {
      let (char_mask, _, char_bit_space) = expected_char_segment.get_value();

      if char_byte & char_mask != char_mask {
        return Err("Buffer has invalid data");
      }

      let mut extract_n_iterate = |bit_count: u8| {
        // WIP: rewriting the below for loop
        let (mask, mask_size) = expected_char_segment.create_data_mask(0, bits_space % BYTE);
        bits_space -= mask_size;
        let bits_left_in_byte = bits_space % BYTE;
        // erm, I need to figure out the shifting for this
        uninit_buffer[buffer_index].write((char_byte & mask) << bits_left_in_byte);

        if bits_left_in_byte != 0 {}
        // WIP
        for bit_index in (0..bit_count).rev() {
          if shift_offset > 0 {
            bits_space = bits_space.saturating_sub(1);
            shift_offset -= 1;
            continue;
          }

          if bits_space > 0 {
            shift_offset = bits_space.min(BYTE);
            buffer_index += 1;
            if buffer_index == MAX_CUSTOM_ID_SIZE {
              return false;
            }
            continue;
          }

          match *schema_segment {
            DataKind::Nullable => {
              // if *deserialized_buffer_slice.index_mut(buffer_index) == 0 {
              //   schema_iterator.next();
              // }
            }
            DataKind::Normal(bits) => {
              bits_space = bits;
              shift_offset = bits.min(8);
            }
            DataKind::Dynamic => {
              // OH FOR FUCKS SAKE, I NEED TO REWRITE EVERYTHING AT THIS POINT
              // awaiting_dynamic_length = true;
              bits_space = 8;
              shift_offset = 8;
            }
            DataKind::Fixed(bytes) => {
              bits_space = bytes * BYTE;
              shift_offset = BYTE;
            }
          }
          buffer_index += 1;
          if buffer_index == MAX_CUSTOM_ID_SIZE {
            return false;
          }
          schema_segment = schema_iterator.next().unwrap();
        }
        true
      };
      if !extract_n_iterate(char_bit_space) {
        break;
      }

      expected_char_segment.next();
    }

    println!("Current buffer_index: {buffer_index}");
    unsafe {
      deserialized_buffer.set_len(buffer_index + 1);
    }
    Ok(deserialized_buffer)
  }

  fn new_deserialize(schema: Vec<u8>, buffer: Vec<u8>) -> Result<Vec<u8>, &'static str> {
    let mut deserialized_buffer: Vec<u8> = Vec::with_capacity(MAX_CUSTOM_ID_SIZE);
    let uninit_buffer = deserialized_buffer.spare_capacity_mut();
    let mut buffer_index: usize = 0;

    let mut data_buffer = DataBuffer::new(parse_schema(&schema)?, buffer);
    // while some data write into uninit buffer
    while let Some(data) = data_buffer.next() {
      uninit_buffer[buffer_index].write(data);
      buffer_index += 1;
    }

    println!("Current buffer_index: {buffer_index}");
    unsafe {
      deserialized_buffer.set_len(buffer_index);
    }
    Ok(deserialized_buffer)
  }
}

#[cfg(test)]
mod test {
  use crate::{DataKind, Deserializer};

  #[test]
  fn new_one_bit() {
    let buffer = vec![0b1111_0010];
    let schema = vec![u8::from(DataKind::Normal(1)), 1];

    let data = Deserializer::new_deserialize(schema, buffer).unwrap();
    assert_eq!(data.len(), 1);

    assert_eq!(data[0], 0);
  }

  // #[test]
  // fn one_bit() {
  //   let serialized_buffer = vec![0b1111_0010];
  //   let schema = vec![u8::from(DataKind::Normal(1)), 1];

  //   let mut deserializer =
  //     Deserializer::new(schema).expect("Deserializer should construct fine with given schema");

  //   let data = deserializer.deserialize(serialized_buffer).unwrap();
  //   assert_eq!(data.len(), 1);

  //   assert_eq!(data[0], 0);
  // }

  // #[test]
  // fn two_bits() {
  //   let serialized_buffer = vec![0b1111_0010, 0b1010_0000];
  //   let schema = vec![u8::from(DataKind::Normal(2)), 2];

  //   let mut deserializer =
  //     Deserializer::new(schema).expect("Deserializer should construct fine with given schema");

  //   let data = deserializer.deserialize(serialized_buffer).unwrap();
  //   assert_eq!(data.len(), 1);

  //   assert_eq!(data[0], 1);
  // }
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
