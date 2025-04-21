use crate::llvm::{types, LikeAFile, Tunnel};

pub fn var(tunnel: &mut Tunnel<'_>, data: (String, [usize; 2])) {
  let lltype: String = types::to_llvm::convert(&data.0);
  tunnel.writeln(
    format!("store {} %{}, ptr %{}, align {}",
      lltype,
      data.1[0],
      data.1[1],
      types::sizeof::calculate(&lltype)
    )
  );
}

pub fn numeric_literal(tunnel: &mut Tunnel<'_>, data: (String, [usize; 2])) {
  let lltype: String = types::to_llvm::convert(&data.0);
  tunnel.writeln(
    format!("store {} {}, ptr %{}, align {}",
      lltype,
      data.1[0],
      data.1[1],
      types::sizeof::calculate(&lltype)
    )
  );
}

pub fn text_literal(tunnel: &mut Tunnel<'_>, data: (String, usize)) {
  tunnel.writeln(
    format!("store {} c\"{}\\00\", ptr %{}, align 1",
      format!("[{} x i8]", data.0.len() + 1),
      data.1,
      data.1
    )
  );
}