use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

use anyhow::Context;

use crate::store::MARKER;

pub enum VaultInit {
    Created(PathBuf),
    AlreadyExists(PathBuf),
}

pub fn run() -> anyhow::Result<()> {
    let cwd = env::current_dir().context("Failed to retrieve the current directory")?;
    let vault_path = cwd.join(MARKER);

    match make_vault_dir(&vault_path)? {
        VaultInit::Created(p) => println!("Initialized todoke vault at {}", p.display()),
        VaultInit::AlreadyExists(p) => println!("Already a todoke vault: {}", p.display()),
    }

    Ok(())
}

// .todoke ディレクトリ作成
fn make_vault_dir(vault_path: &Path) -> anyhow::Result<VaultInit> {
    let path = vault_path.to_path_buf();

    match fs::create_dir(&path) {
        Ok(()) => Ok(VaultInit::Created(path)),
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
            if !vault_path.is_dir() {
                anyhow::bail!(
                    "{} already exists but is not a directory",
                    vault_path.display()
                );
            }
            Ok(VaultInit::AlreadyExists(path))
        }
        Err(e) => Err(e).with_context(|| {
            format!(
                "Failed to create the vault directory: {}",
                vault_path.display()
            )
        }),
    }
}
