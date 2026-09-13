use std::path::Path;

use crate::model::slug::Slug;

pub fn run(_data_dir: &Path, _slug: Slug, _template: String) -> anyhow::Result<()> {
    anyhow::bail!("todoke render is not implemented yet")
}
