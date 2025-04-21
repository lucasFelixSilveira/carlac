use crate::{llvm::{types, LikeAFile, Tunnel}, parser::Definition};

pub fn create(tunnel: &mut Tunnel<'_>, data: &Definition) {
  let lltype: String = types::to_llvm::convert(&data.ctype);
  tunnel.writeln(
    format!("%{} = alloca {lltype}, align {}",
      tunnel.var,
      types::sizeof::calculate(&lltype)
    )
  );
  *tunnel.var += 1;
}

pub fn literal(tunnel: &mut Tunnel<'_>, ctype: impl ToString) {
  let lltype: String = types::to_llvm::convert(&ctype.to_string());
  tunnel.writeln(
    format!("%{} = alloca {lltype}, align {}",
      tunnel.var,
      types::sizeof::calculate(&lltype)
    )
  );
  *tunnel.var += 1;
}