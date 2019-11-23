fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("src/generated")
        .build_server(false)
        .compile(
            &["proto/grpc.proto"],
            &["proto/"],
        )?;
    Ok(())
}
