fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .compile(
            &["../cast-mesh-server/proto/counter.proto"],
            &["../cast-mesh-server/proto/"],
        )?;

    Ok(())
}