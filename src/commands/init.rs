use std::{
    env,
    fs::{self},
    io,
    path::{Path, PathBuf},
};

use anyhow::Context;

pub enum VaultInit {
    Created(PathBuf),
    AlreadyExists(PathBuf),
}

pub fn run() -> anyhow::Result<()> {
    let cwd = env::current_dir().context("Failed to retrieve the current directory")?;

    match make_vault_dir(&cwd)? {
        VaultInit::Created(p) => println!("Initialized todoke vault at {}", p.display()),
        VaultInit::AlreadyExists(p) => println!("Already a todoke vault: {}", p.display()),
    }

    Ok(())
}

// .todoke ディレクトリ作成
fn make_vault_dir(base: &Path) -> anyhow::Result<VaultInit> {
    let vault_path = base.join(".todoke");
    match fs::create_dir(&vault_path) {
        Ok(()) => Ok(VaultInit::Created(vault_path)),
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
            if !vault_path.is_dir() {
                anyhow::bail!(
                    "{} already exists but is not a directory",
                    vault_path.display()
                );
            }
            Ok(VaultInit::AlreadyExists(vault_path))
        }
        Err(e) => Err(e).with_context(|| {
            format!(
                "Failed to create the vault directory: {}",
                vault_path.display()
            )
        }),
    }
}
