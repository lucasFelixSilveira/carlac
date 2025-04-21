use num_bigint::BigUint;
use num_traits::Zero;

use crate::llvm::{LikeAFile, Tunnel};

pub fn string_to_biguint_and_bits(s: &str) -> (BigUint, usize) {
  let bytes = s.as_bytes();
  let mut value = BigUint::zero();

  for &b in bytes {
    value = (value << 8) | BigUint::from(b);
  }

  let bits = bytes.len() * 8;
  (value, bits)
}

pub fn string_literal(tunnel: &mut Tunnel<'_>, ctx: impl ToString) {
  let (integer, bits) = string_to_biguint_and_bits(&ctx.to_string());

  tunnel.shard.writeln(
    format!("%{} = inttoptr i{bits} {integer} to i8*", 
      *tunnel.var
    )
  );
  *tunnel.var += 1;
}