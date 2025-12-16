// Build script for Cap'n Proto schema compilation

fn main() {
    // Only compile Cap'n Proto if the schema exists and capnp is available
    if std::path::Path::new("schema/user.capnp").exists() {
        if let Err(e) = capnpc::CompilerCommand::new()
            .src_prefix("schema")
            .file("schema/user.capnp")
            .run()
        {
            // Print warning but don't fail the build
            println!("cargo:warning=Cap'n Proto schema compilation skipped: {:?}", e);
            println!("cargo:warning=Install Cap'n Proto to enable Cap'n Proto benchmarks: choco install capnproto");
        }
    }
}
