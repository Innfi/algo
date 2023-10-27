use tonic_build::compile_protos;
use std::io::Result;

fn main() -> Result<()> {
  compile_protos("src/contract.proto")?;

  Ok(())
}