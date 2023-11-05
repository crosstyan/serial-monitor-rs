fn main() -> Result<(), Box<dyn std::error::Error>> {
    let builder = tonic_build::configure()
                        .build_client(false)
                        .build_server(true)
                        .out_dir("src/serial/api");
    let protos = &["api/proto/api.proto"];
    let includes = &["api/proto"];
    builder.compile(protos, includes)?;
    Ok(())
}
