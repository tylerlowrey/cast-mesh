fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir("src/");
    tonic_build::configure()
        .build_client(false)
        .out_dir("src/")
        .compile(
            &["../cast-mesh-core/proto/pingpong.proto"],
            &["../cast-mesh-core/proto/"],
        )?;
    Ok(())
}