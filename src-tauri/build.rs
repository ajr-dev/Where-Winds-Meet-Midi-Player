fn main() {
    #[cfg(windows)]
    {
        let mut windows = tauri_build::WindowsAttributes::new();
        
        // Only apply admin manifest in release builds
        // Debug builds can run without it (hooks won't work, but app will start)
        #[cfg(not(debug_assertions))]
        {
            windows = windows.app_manifest(include_str!("app.manifest"));
        }
        
        tauri_build::try_build(tauri_build::Attributes::new().windows_attributes(windows))
            .expect("failed to run build script");
    }

    #[cfg(not(windows))]
    tauri_build::build();
}