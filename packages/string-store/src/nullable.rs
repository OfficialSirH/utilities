use crate::{DataKind, Deserializer, Handler};

pub struct Nullable {}

// impl Handler<Nullable> for Deserializer {
//   fn handle(
//     &self,
//     // buffer: &mut [u8],
//     // index: &mut u8,
//     // ptr
//     serialized_buffer: &mut [u8],
//   ) -> Result<bool, &'static str> {
//     // carried_bits
//     todo!()
//   }
// }

// nullable needs to check if its bit is 1 or 0
// it obtains this information by extracting a bit from the current char_byte
// extracting a bit would require knowing the current offset and number of bits left to fulfill the task
// knowing the current offset would require knowing previous/initial offset
// knowing the number of leftover bits is done within function
// after obtaining the bit, if the bit is 0, skip the following schema kind (return true)
// in any case, modify the buffer with whichever bit value and increment the index.
