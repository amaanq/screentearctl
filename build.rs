use std::path::Path;

use clap::{Command, CommandFactory};
use clap_complete::{
    generate_to,
    shells::{Bash, Elvish, Fish, PowerShell, Zsh},
};

include!("src/cli.rs");

fn build_cli() -> Command {
    let mut app = ScreenTearArgs::command();
    app.set_bin_name("screentearctl");
    app
}

fn main() -> Result<(), std::io::Error> {
    let mut app = build_cli();

    app.set_bin_name("screentearctl");

    let outdir = Path::new(env!("CARGO_MANIFEST_DIR")).join("completions/");

    generate_to(Bash, &mut app, "screentearctl", &outdir)?;
    generate_to(Elvish, &mut app, "screentearctl", &outdir)?;
    generate_to(Fish, &mut app, "screentearctl", &outdir)?;
    generate_to(Zsh, &mut app, "screentearctl", &outdir)?;
    generate_to(PowerShell, &mut app, "screentearctl", &outdir)?;

    Ok(())
}
