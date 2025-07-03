pub trait Handler<T> {
  fn handle(&self, buffer: &mut [u8], index: &mut u8) -> Result<bool, &'static str>;
}
