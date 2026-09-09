mod event;
mod init;
mod list;
mod new;
mod render;
mod status;

use crate::{
    cli::{Args, Command},
    store::{MARKER, discover_root},
};

pub fn dispatch(cli: Args) -> anyhow::Result<()> {
    match cli.command {
        Command::Init => init::run(),

        other => {
            let cwd = std::env::current_dir()?;
            let root = discover_root(&cwd).ok_or_else(|| {
                anyhow::anyhow!("not inside a todoke vault (run `todoke init` first)")
            })?;
            let data_dir = root.join(MARKER);

            match other {
                Command::New { title, slug } => new::run(&data_dir, title, slug),
                Command::Status { slug } => status::run(&data_dir, slug),
                Command::List => list::run(&data_dir),
                Command::Render { slug, template } => render::run(&data_dir, slug, template),
                Command::Event {
                    slug,
                    kind,
                    date,
                    note,
                    reference,
                } => event::run(&data_dir, slug, kind, date, note, reference),
                Command::Init => unreachable!(), // 処理済みなため到達不可
            }
        }
    }
}
