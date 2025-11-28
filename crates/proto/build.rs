use prost_build::Config;
use std::io::Result;

fn main() -> Result<()> {
    let mut config = Config::new();

    config.compile_protos(&["protos/auth.proto"], &["protos/"])?;
    Ok(())
}
