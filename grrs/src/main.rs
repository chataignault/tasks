use anyhow::{Context, Result};
use clap::Parser;
use indicatif::ProgressBar;
use std::io::{self, Write};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could read file `{}`", args.path.display()))?;

    let bar = ProgressBar::new_spinner();
    for line in content.lines() {
        if line.contains(&args.pattern) {
            writeln!(handle, "{}", line)?;
        }
        bar.tick();
    }

    handle.flush().unwrap();
    Ok(())
}
