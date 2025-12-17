// Build script for Cap'n Proto schema compilation

fn main() {
    // Only compile Cap'n Proto if the schema exists and capnp is available
    if std::path::Path::new("schema/user.capnp").exists() {
        // Try to find capnp in common locations
        let userprofile = std::env::var("USERPROFILE").unwrap_or_default();
        let winget_path = format!("{}\\AppData\\Local\\Microsoft\\WinGet\\Packages\\capnproto.capnproto_Microsoft.Winget.Source_8wekyb3d8bbwe\\capnproto-tools-win32-1.1.0\\capnp.exe", userprofile);
        
        let capnp_paths = vec![
            "capnp",  // In PATH
            "C:\\Program Files\\capnproto\\bin\\capnp.exe",
            winget_path.as_str(),
        ];
        
        let capnp_found = capnp_paths.iter().any(|path| {
            std::process::Command::new(path)
                .arg("--version")
                .output()
                .is_ok()
        });
        
        if !capnp_found {
            println!("cargo:warning=Cap'n Proto not found in PATH or common locations");
            println!("cargo:warning=Install Cap'n Proto: winget install capnproto.capnproto");
            return;
        }
        
        if let Err(e) = capnpc::CompilerCommand::new()
            .src_prefix("schema")
            .file("schema/user.capnp")
            .run()
        {
            println!("cargo:warning=Cap'n Proto schema compilation failed: {:?}", e);
        }
    }
}
