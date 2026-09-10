use std::path::Path;

use crate::{
    model::slug::Slug,
    store::{DirInit, make_dir},
};

pub fn run(vault_dir: &Path, _title: String, slug: Slug) -> anyhow::Result<()> {
    let case_dir = vault_dir.join(slug.as_str());

    match make_dir(&case_dir)? {
        DirInit::Created(p) => println!("Created case at {}", p.display()),
        DirInit::AlreadyExists(_) => anyhow::bail!("case {:?} already exists", slug),
    }

    Ok(())
}

