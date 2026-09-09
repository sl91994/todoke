use std::path::{Path, PathBuf};

pub const MARKER: &str = ".todoke";

/// カレントから上へ辿って .todoke を持つ vault_root を探す
pub fn discover_root(start: &Path) -> Option<PathBuf> {
    start
        .ancestors()
        .find(|dir| dir.join(MARKER).is_dir())
        .map(Path::to_path_buf)
}
