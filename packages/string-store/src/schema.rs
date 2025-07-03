use crate::DataKind;

pub struct Schema {
  pub data: Vec<DataKind>,
  pub offset: u8,
}

impl Schema {
  pub fn next_bit() {}
}
