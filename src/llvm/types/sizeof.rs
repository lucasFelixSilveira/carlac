pub fn calculate(string: &String) -> usize {
  if string.starts_with('i') {
    let bits:  String = string[1..string.len()].to_string();
    let i_bits: usize = bits.parse::<usize>().unwrap();
    return i_bits / 8;
  }

  if string == "ptr" { std::mem::size_of::<usize>() } 
  else { 0 }
}