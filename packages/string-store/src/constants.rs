/// 20 subtracting one bit to ensure the character is within the valid range
pub const CHAR_BIT_SPACE: usize = 19;
/// The amount of available bit space in a continuation byte
pub const BITS_IN_CONTINUATION_BYTES: usize = 6;
/// The assumed size for all generated characters in this library
pub const CHAR_SIZE: usize = 4;
/// The max amount of bytes a dynamically-sized serializable data entry can be
///   before it (with the schema ID combined) exceeds the max available bits
///
/// byte mask: `0b1110_1100` -> unsigned 8-bit integer `236` -> signed 8-bit integer `-20`
pub const MAX_SAFE_DYNAMIC_BYTE_LENGTH: u8 = 0b1110_1100;
/// Used as a prefix byte in the schema bit size data to indicate to the serializer/deserializer
/// that the given data has a prefixed length for the data's byte count.
pub const DYNAMIC_DATA_INDICATOR: u8 = MAX_SAFE_DYNAMIC_BYTE_LENGTH + 1;
/// Max size (in bytes) that a discord component's custom_id can contain
pub const MAX_CUSTOM_ID_SIZE: usize = 400;
pub const BYTE: u8 = 8;
