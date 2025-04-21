pub fn to_utf16_null_terminated(s: &str) -> Vec<u16> {
  let mut utf16: Vec<u16> = s.encode_utf16().collect();
  utf16.push(0);
  utf16
}
