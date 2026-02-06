use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "dawn-gen",
    version,
    about = "Generate Rust API bindings from dawn.json"
)]
struct Cli {
    #[arg(long, default_value = "dawn.json")]
    input: PathBuf,

    #[arg(long, default_value = "generated")]
    out_dir: PathBuf,

    #[arg(long, value_delimiter = ',', default_value = "")]
    tags: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let tags: Vec<String> = cli
        .tags
        .into_iter()
        .filter(|tag| !tag.trim().is_empty())
        .collect();

    dawn_rs::generate(cli.input, cli.out_dir, &tags)?;
    Ok(())
}
