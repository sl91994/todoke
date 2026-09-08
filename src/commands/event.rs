use std::path::Path;

use crate::cli::EventKind;

pub fn run(
    _data_dir: &Path,
    _slug: String,
    _kind: EventKind,
    _date: Option<String>,
    _note: Option<String>,
    _reference: Option<String>,
) -> anyhow::Result<()> {
    todo!()
}
