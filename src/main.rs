use std::{process, fmt};
use colored::Colorize;
use clap::{Parser, Subcommand, ValueEnum};
use pinenote::ebc::Waveform;

use crate::pinenote::*;

mod interfaces;
mod pinenote;



#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Await(AwaitArgs),
    FullRefresh(FullRefreshArgs),
    PerformanceMode(#[clap(flatten)] PerformanceModeArgs),
    Waveform(#[clap(flatten)] WaveformArgs),
}

#[derive(Parser, Debug)]
struct AwaitArgs {
    target: AwaitTarget,

    #[clap(short, long)]
    r#loop: bool,
}

#[derive(Parser, Debug)]
struct FullRefreshArgs {}

#[derive(Parser, Debug)]
struct PerformanceModeArgs {
    action: Option<OnOffToggleState>,
}

#[derive(Parser, Debug)]
struct WaveformArgs {
    waveform_opt: Option<Waveform>,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
enum AwaitTarget {
    PerformanceModeChanged,
    WaveformChanged,
}


fn resolve<T, E: fmt::Display>(result: Result<T, E>) -> T {
    match result {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", format!("Error: {}", e).red());
            process::exit(1);
        },
    }
}

impl Command {
    fn exec(self, pinenote: Pinenote) -> Result<(), String> {
        use Command::*;
        match self {
            Await(args) => Self::r#await(args, pinenote),
            FullRefresh(_) => Self::refresh(pinenote),
            PerformanceMode(args) => Self::performance_mode(args, pinenote),
            Waveform(args) => Self::waveform(args, pinenote),
        }
    }

    fn performance_mode(args: PerformanceModeArgs, pinenote: Pinenote) -> Result<(), String> {
        if let Some(a) = args.action {
            pinenote.ebc().change_performance_mode(a)?;
        }
        pinenote.ebc().print_performance_mode()?;
        Ok(())
    }

    fn waveform(args: WaveformArgs, pinenote: Pinenote) -> Result<(), String> {
        if let Some(wv) = args.waveform_opt {
            pinenote.ebc().set_waveform(wv)?;
        }
        pinenote.ebc().print_waveform()?;
        Ok(())
    }


    fn refresh(pinenote: Pinenote) -> Result<(), String> {
        pinenote.ebc().full_refresh()
    }

    fn r#await(args: AwaitArgs, pinenote: Pinenote) -> Result<(), String> {
        loop {
            use AwaitTarget::*;
            match args.target {
                PerformanceModeChanged => pinenote.ebc().await_performance_mode_change()?,
                WaveformChanged => pinenote.ebc().await_waveform_change()?,
            }

            match args.target {
                PerformanceModeChanged => pinenote.ebc().print_performance_mode()?,
                WaveformChanged => pinenote.ebc().print_waveform()?,
            }

            if !args.r#loop {
                break;
            }
        }
        Ok(())
    }
}

fn main() {
    let args = Args::parse();
    let pinenote = resolve(Pinenote::new());
    resolve(args.command.exec(pinenote));
}
