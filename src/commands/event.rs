use std::path::Path;

use crate::{cli::EventKind, model::slug::Slug};

pub fn run(
    _data_dir: &Path,
    _slug: Slug,
    _kind: EventKind,
    _date: Option<String>,
    _note: Option<String>,
    _reference: Option<String>,
) -> anyhow::Result<()> {
    anyhow::bail!("todoke event is not implemented yet")
}
