fn main() -> anyhow::Result<()> {
    prost_build::compile_protos(&["./protocol/client.proto"], &["./protocol/"])?;
    prost_build::compile_protos(&["./protocol/server.proto"], &["./protocol/"])?;
    Ok(())
}
