use std::{fmt, ops};

use zbus::blocking::Connection;


pub mod ebc;


pub struct Pinenote<'a> {
    dbus_connection: Connection,
    ebc: ebc::Ebc<'a>,
}

#[derive(clap::ValueEnum, Copy, Clone, Debug)]
pub enum OnOffToggleState {
    On,
    Off,
    Toggle
}

#[derive(clap::ValueEnum, Copy, Clone, Debug)]
pub enum OnOffState {
    On,
    Off,
}


impl<'a> Pinenote<'a> {
    pub fn new() -> Result<Self, String> {
        let dbus_connection = Connection::system()
            .map_err(|e| e.to_string())?;
        let ebc = ebc::Ebc::new(&dbus_connection)?;

        Ok(Pinenote { dbus_connection, ebc })
    }

    pub fn ebc(&self) -> &ebc::Ebc {
        &self.ebc
    }
}


impl ops::Not for OnOffState {
    type Output = Self;
    fn not(self) -> Self {
        use OnOffState::*;
        match self {
            On => Off,
            Off => On,
        }
    }
}

impl From<OnOffState> for bool {
    fn from(state: OnOffState) -> bool {
        use OnOffState::*;
        match state {
            On => true,
            Off => false,
        }
    }
}

impl From<bool> for OnOffState {
    fn from(val: bool) -> OnOffState {
        use OnOffState::*;
        match val {
            true => On,
            false => Off,
        }
    }
}

impl From<OnOffState> for OnOffToggleState {
    fn from(state: OnOffState) -> OnOffToggleState {
        use OnOffState::*;
        match state {
            On => OnOffToggleState::On,
            Off => OnOffToggleState::Off,
        }
    }
}

impl TryFrom<OnOffToggleState> for OnOffState {
    type Error = String;
    fn try_from(state: OnOffToggleState) -> Result<OnOffState, String> {
        use OnOffToggleState::*;
        match state {
            On => Ok(OnOffState::On),
            Off => Ok(OnOffState::Off),
            Toggle => Err(format!("Attempting to conver 'Toggle' value to OnOffState")),
        }
    }
}

impl fmt::Display for OnOffState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use OnOffState::*;
        match self {
            On => write!(f, "on"),
            Off => write!(f, "off"),
        }
    }
}
