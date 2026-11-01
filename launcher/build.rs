//! Embeds Coconuted's icon (`packaging/icons/coconuted.ico`) in the Windows executable, as icon
//! resource 1: Explorer and the task bar show it, and the window takes it as its icon.
fn main() {
    println!("cargo:rustc-check-cfg=cfg(coconuted_embedded_client)");
    println!("cargo:rerun-if-env-changed=COCONUTED_CLIENT_EXE");
    if let Ok(client) = std::env::var("COCONUTED_CLIENT_EXE") {
        assert!(std::path::Path::new(&client).is_file(), "missing client executable: {client}");
        println!("cargo:rerun-if-changed={client}");
        println!("cargo:rustc-env=COCONUTED_CLIENT_EXE={client}");
        println!("cargo:rustc-cfg=coconuted_embedded_client");
    }
    let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
    let ico = format!("{dir}/../packaging/icons/coconuted.ico");
    println!("cargo:rerun-if-changed={ico}");
    #[cfg(windows)]
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let mut resource = winresource::WindowsResource::new();
        resource.set_icon(&ico);
        resource.set("ProductName", "Coconuted Launcher");
        resource.set("FileDescription", "Coconuted Launcher");
        if let Err(e) = resource.compile() {
            panic!("embedding {ico}: {e}");
        }
    }
}
