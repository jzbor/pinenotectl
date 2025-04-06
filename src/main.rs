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
    /// Wait for a certain event
    Await(AwaitArgs),

    /// Do a full refresh of the eink screen
    FullRefresh(FullRefreshArgs),

    /// Print information on all available parameters
    Info(InfoArgs),

    /// Manage eink panel performance mode
    ///
    /// Performance mode brings faster refreshes, but may be prone to fragments
    /// and a lower image quality overall.
    PerformanceMode(#[clap(flatten)] PerformanceModeArgs),

    /// Manage travel mode (disables lid wakeup)
    TravelMode(#[clap(flatten)] TravelModeArgs),

    /// Manage eink panel waveform
    Waveform(#[clap(flatten)] WaveformArgs),
}

#[derive(Parser, Debug)]
struct AwaitArgs {
    /// Event to wait for
    target: AwaitTarget,

    /// Wait in a loop, outputting a new line at every event
    #[clap(short, long)]
    r#loop: bool,
}

#[derive(Parser, Debug)]
struct FullRefreshArgs {}

#[derive(Parser, Debug)]
struct InfoArgs {}

#[derive(Parser, Debug)]
struct PerformanceModeArgs {
    /// Change current performance mode setting
    action: Option<OnOffToggleState>,
}

#[derive(Parser, Debug)]
struct TravelModeArgs {
    /// Change current travel mode setting
    action: Option<OnOffToggleState>,
}

#[derive(Parser, Debug)]
struct WaveformArgs {
    /// Waveform setting
    waveform_opt: Option<Waveform>,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
enum AwaitTarget {
    PerformanceModeChanged,
    TravelModeChanged,
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
            Info(_) => Self::info(pinenote),
            PerformanceMode(args) => Self::performance_mode(args, pinenote),
            TravelMode(args) => Self::travel_mode(args, pinenote),
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

    fn travel_mode(args: TravelModeArgs, pinenote: Pinenote) -> Result<(), String> {
        if let Some(a) = args.action {
            pinenote.change_travel_mode(a)?;
        }
        pinenote.print_travel_mode()?;
        Ok(())
    }

    fn waveform(args: WaveformArgs, pinenote: Pinenote) -> Result<(), String> {
        if let Some(wv) = args.waveform_opt {
            pinenote.ebc().set_waveform(wv)?;
        }
        pinenote.ebc().print_waveform()?;
        Ok(())
    }

    fn info(pinenote: Pinenote) -> Result<(), String> {
        pinenote.print_travel_mode()?;
        pinenote.ebc().print_waveform()?;
        pinenote.ebc().print_performance_mode()?;
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
                TravelModeChanged => pinenote.await_travel_mode_change()?,
                WaveformChanged => pinenote.ebc().await_waveform_change()?,
            }

            match args.target {
                PerformanceModeChanged => pinenote.ebc().print_performance_mode()?,
                TravelModeChanged => pinenote.print_travel_mode()?,
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
