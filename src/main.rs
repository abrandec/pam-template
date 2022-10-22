mod misc;
// local
use misc::{
    cli::{Opts, Subcommands},
    generator::gen_template,
};
// external
use clap::Parser;
use rust_embed::RustEmbed;
use std::str::from_utf8;

pub fn main() {
    let opts = Opts::parse();

    match opts.sub {
        Subcommands::Create(argy) => {
            let project_name: String = argy
                .lib
                .clone()
                .expect("no string provided for arguement 😔");
            gen_template(project_name);
        }
    }
}
