fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=migrations");
    tonic_build::configure()
        .type_attribute(".", "#[derive(sqlx::FromRow)]")
        .compile(&["../proto/splitter.proto"], &["../proto"])
        .unwrap();
    Ok(())
}
