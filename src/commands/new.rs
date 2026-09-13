use std::path::Path;

use anyhow::Context;
use chrono::Utc;

use crate::{
    model::slug::Slug,
    store::{PathInit, make_dir, make_file, toml_store::Case},
};

pub fn run(vault_dir: &Path, title: String, slug: Slug) -> anyhow::Result<()> {
    // case_dir を新規作成
    let slug_str = slug.as_str();
    let case_dir = vault_dir.join(slug_str);

    match make_dir(&case_dir)? {
        PathInit::Created(p) => println!("Created case at {}", p.display()),
        PathInit::AlreadyExists(_) => anyhow::bail!("case {:?} already exists", slug),
    }

    // case_dir 内に case.toml のひな型を作成
    let created_at = Utc::now().date_naive().to_string();
    let template = Case::new(&slug, title, created_at);
    let template_toml = toml::to_string(&template).context("Failed to parse case.toml")?;
    let template_path = case_dir.join("case.toml");

    match make_file(&template_path, template_toml.as_bytes())? {
        PathInit::Created(p) => println!("Created case.toml at {}", p.display()),
        PathInit::AlreadyExists(_) => anyhow::bail!("case.toml {:?} already exists", slug),
    }

    Ok(())
}
