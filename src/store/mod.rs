pub mod toml_store;

use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
};

use anyhow::Context;

pub const MARKER: &str = ".todoke";

pub enum PathInit {
    Created(PathBuf),
    AlreadyExists(PathBuf),
}

/// カレントから上へ辿って .todoke を持つ vault_root を探す
pub fn discover_root(start: &Path) -> Option<PathBuf> {
    start
        .ancestors()
        .find(|dir| dir.join(MARKER).is_dir())
        .map(Path::to_path_buf)
}

/// ディレクトリを作成する (vault / case 共通)
///
/// 既に同名のディレクトリがある場合は `AlreadyExists` を返し，
/// ディレクトリ以外が存在する場合はエラー
pub fn make_dir(dir: &Path) -> anyhow::Result<PathInit> {
    let path = dir.to_path_buf();

    match fs::create_dir(&path) {
        Ok(()) => Ok(PathInit::Created(path)),
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
            if !dir.is_dir() {
                anyhow::bail!("{} already exists but is not a directory", dir.display());
            }
            Ok(PathInit::AlreadyExists(path))
        }
        Err(e) => {
            Err(e).with_context(|| format!("Failed to create the directory: {}", dir.display()))
        }
    }
}
