use crate::constants::BYTE;

#[derive(PartialEq)]
pub enum DataKind {
  Nullable,
  // contains bit size of data type
  Normal(u8),
  Dynamic,
  // contains byte length of fixed-sized data type
  Fixed(u8),
}

impl DataKind {
  pub fn get_bits_n_offset(&self) -> (u8, u8) {
    match self {
      DataKind::Nullable => (1, 0),
      DataKind::Normal(bits) => (*bits, (*bits).min(BYTE - 1)),
      DataKind::Dynamic => (BYTE, BYTE - 1),
      DataKind::Fixed(byte_length) => (*byte_length * BYTE, BYTE - 1),
    }
  }

  pub fn get_size(&self) -> u8 {
    self.get_bits_n_offset().0
  }
}

impl TryFrom<u8> for DataKind {
  type Error = &'static str;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    match value {
      0 => Ok(Self::Nullable),
      1 => Ok(Self::Normal(0)),
      2 => Ok(Self::Fixed(0)),
      3 => Ok(Self::Dynamic),
      _ => {
        Err("Unknown data type received. Expected: [0(Nullable), 1(Normal), 2(Fixed), 3(Dynamic)]")
      }
    }
  }
}

impl From<DataKind> for u8 {
  fn from(value: DataKind) -> Self {
    match value {
      DataKind::Nullable => 0,
      DataKind::Normal(_) => 1,
      DataKind::Fixed(_) => 2,
      DataKind::Dynamic => 3,
    }
  }
}
