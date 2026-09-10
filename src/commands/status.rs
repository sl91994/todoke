use std::path::Path;

use crate::model::slug::Slug;

pub fn run(_data_dir: &Path, _slug: Slug) -> anyhow::Result<()> {
    anyhow::bail!("todoke status is not implemented yet")
}
