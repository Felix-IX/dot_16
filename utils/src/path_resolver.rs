use std::path::PathBuf;

/// resolve from the workspace root path
pub fn path_from_workspace_root(relative_path: &str) -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let mut path = PathBuf::from(manifest_dir);
    while !path.join("assets").exists() && path.pop() {}
    path.join(relative_path)
}
