use std::path::PathBuf;

use clap::Parser;
use delete_images_from_zips::run;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// Root directory to scan
    #[arg(value_name = "DIR", required_unless_present = "clear_cache")]
    root: Option<PathBuf>,

    /// Keywords separated by comma
    #[arg(long, value_name = "kw1,kw2", required_unless_present = "clear_cache")]
    keywords: Option<String>,

    /// Show progress to stderr
    #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
    progress: bool,

    /// Clear cache file and exit
    #[arg(long)]
    clear_cache: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    if cli.clear_cache {
        delete_images_from_zips::clear_cache_file().map_err(|e| anyhow::anyhow!(e))?;
        return Ok(());
    }
    let root = cli
        .root
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("DIR is required unless --clear-cache"))?;
    let keywords = cli
        .keywords
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("--keywords is required unless --clear-cache"))?;
    run(root, keywords, cli.progress).map_err(|e| anyhow::anyhow!(e))
}
