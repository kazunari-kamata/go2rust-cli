use anyhow::Context;
use clap::Parser;
use go2rust_cli::cli::{Cli, Commands};
use std::fs;
use std::io::{self, Write};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let Commands::Convert {
        input,
        output,
        check,
    } = cli.command;

    let source = fs::read_to_string(&input)
        .with_context(|| format!("failed to read input file: {}", input.display()))?;
    let rust_code = go2rust_cli::convert_source(&source)?;

    if check {
        println!("OK");
        return Ok(());
    }

    match output {
        Some(path) => {
            fs::write(&path, rust_code)
                .with_context(|| format!("failed to write output file: {}", path.display()))?;
        }
        None => {
            let mut stdout = io::stdout().lock();
            stdout.write_all(rust_code.as_bytes())?;
        }
    }

    Ok(())
}
