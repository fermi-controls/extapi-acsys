fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=src/g_rpc/dpm/deviceinfo.proto");
    tonic_build::compile_protos("src/g_rpc/dpm/dpm.proto")?;
    Ok(())
}
