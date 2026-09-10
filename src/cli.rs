use clap::{Parser, Subcommand, ValueEnum};

use crate::model::slug::Slug;

#[derive(ValueEnum, Clone, Debug)]
pub enum EventKind {
    Discovered,
    Reported,
    Acknowledged,
    Fixed,
    Disclosed,
    /// etc...
    Note,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Emit machine-readable JSON instead of human output
    #[arg(long, global = true)]
    pub json: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Initialize a todoke vault in the current directory (.git style)
    Init,
    /// Create a new disclosure case
    New {
        /// Short title of the vulnerability
        title: String,
        /// Directory-safe identifier
        slug: Slug,
    },
    /// Append an event to a case timeline
    Event {
        /// Case slug
        slug: Slug,
        /// Event kind (discovered, reported, fixed, disclosed, ...)
        kind: EventKind,
        /// Date of the event (YYYY-MM-DD; defaults to today)
        #[arg(long)]
        date: Option<String>,
        /// Free-form note
        #[arg(long)]
        note: Option<String>,
        /// Reference such as a CVE, GHSA or JVN id
        #[arg(long = "ref")]
        reference: Option<String>,
    },
    /// Show a single case and its timeline
    Status {
        /// Case slug
        slug: Slug,
    },
    /// List all cases
    List,
    /// Render a report from a template
    Render {
        /// Case slug
        slug: Slug,
        /// Template name (ghsa, ipa, blog)
        #[arg(long)]
        template: String,
    },
}
