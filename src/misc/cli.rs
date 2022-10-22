use clap::{Parser, Subcommand, ValueHint};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(name = "PAM Rust Template", version = "0.0.1")]
pub struct Opts {
    #[clap(subcommand)]
    pub sub: Subcommands,
}

#[derive(Debug, Subcommand)]
#[clap(
    about = "Generate a PAM project with a module + client",
    next_display_order = None
)]
#[allow(clippy::large_enum_variant)]
pub enum Subcommands {
    #[clap(about = "Create a project with a PAM module & client.")]
    Create(CreateArgs),
}

#[derive(Debug, Clone, Parser)]
pub struct CreateArgs {
    #[clap(
        help = "Create a PAM project that contains a module & client.",
        value_hint = ValueHint::DirPath
    )]
    pub lib: Option<String>,
}
