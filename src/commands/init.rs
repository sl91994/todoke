use std::env;

use anyhow::Context;

use crate::store::{DirInit, MARKER, make_dir};

pub fn run() -> anyhow::Result<()> {
    let cwd = env::current_dir().context("Failed to retrieve the current directory")?;
    let vault_path = cwd.join(MARKER);

    match make_dir(&vault_path)? {
        DirInit::Created(p) => println!("Initialized todoke vault at {}", p.display()),
        DirInit::AlreadyExists(p) => println!("Already a todoke vault: {}", p.display()),
    }

    Ok(())
}
