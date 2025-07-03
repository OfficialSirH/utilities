use std::slice::Iter;

use crate::DataKind;

pub fn parse_schema(schema: &[u8]) -> Result<Vec<DataKind>, &'static str> {
  let mut parsed_schema = Vec::new();

  let mut schema_iterator = schema.iter();
  while let Some(value) = schema_iterator.next() {
    parsed_schema.push(parse_kind(&mut schema_iterator, *value, false)?);
  }

  return Ok(parsed_schema);
}

pub fn parse_kind(
  schema_iterator: &mut Iter<'_, u8>,
  value: u8,
  recursive: bool,
) -> Result<DataKind, &'static str> {
  let data = DataKind::try_from(value)?;
  match data {
    // DataKind::Nullable(_) => {
    //   if recursive {
    //     return Err("Nullable within Nullable isn't allowed");
    //   }
    //   let value_type = *schema_iterator
    //     .next()
    //     .ok_or("Missing data type for nullable.")?;

    //   Ok(DataKind::Nullable(Box::new(parse_kind(
    //     schema_iterator,
    //     value_type,
    //     true,
    //   )?)))
    // }
    DataKind::Nullable | DataKind::Dynamic => Ok(data),
    DataKind::Normal(_) => {
      let bit_size = *schema_iterator
        .next()
        .ok_or("Missing bit size for normal type.")?;

      Ok(DataKind::Normal(bit_size))
    }
    DataKind::Fixed(_) => {
      let byte_length = *schema_iterator
        .next()
        .ok_or("Missing byte length for fixed type.")?;

      Ok(DataKind::Fixed(byte_length))
    }
  }
}
