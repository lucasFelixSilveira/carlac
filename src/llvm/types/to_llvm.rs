pub fn convert(original: &String) -> String {
  if original.starts_with('&') || original.starts_with('[') {
    return "ptr".into();
  }

  if original == "usize" { 
    return ["i", &(std::mem::size_of::<usize>() * 8).to_string()].concat();
  }

  match original.as_str() {
      "int8"   |  "uint8" 
    | "char"   |  "ascii" 
    | "byte"   => "i8",
    
    "int16"    |  "uint16"  => "i16",
    "int32"    |  "uint32"  => "i32",
    "int64"    |  "uint64"  => "i64",
    "int128"   |  "uint128" => "i128",
    "int256"   |  "uint256" => "i256",
    "void"     => "void",

    _ => "void"

  }.into()
}