use std::{fmt, ops};

use zbus::blocking::Connection;

use crate::interfaces::misc::MiscProxyBlocking;


pub mod ebc;


pub struct Pinenote<'a> {
    #[allow(dead_code)]
    dbus_connection: Connection,
    ebc: ebc::Ebc<'a>,
    misc_interface: MiscProxyBlocking<'a>,
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


impl Pinenote<'_> {
    pub fn new() -> Result<Self, String> {
        let dbus_connection = Connection::system()
            .map_err(|e| e.to_string())?;
        let ebc = ebc::Ebc::new(&dbus_connection)?;
        let misc_interface = MiscProxyBlocking::new(&dbus_connection)
            .map_err(|e| e.to_string())?;

        Ok(Pinenote { dbus_connection, ebc, misc_interface })
    }

    pub fn ebc(&self) -> &ebc::Ebc {
        &self.ebc
    }

    pub fn set_travel_mode(&self, state: OnOffState) -> Result<(), String> {
        use OnOffState::*;
        match state {
            On => self.misc_interface.enable_travel_mode(),
            Off => self.misc_interface.disable_travel_mode(),
        }.map_err(|e| e.to_string())
    }

    pub fn get_travel_mode(&self) -> Result<OnOffState, String> {
        use OnOffState::*;
        let val = self.misc_interface.get_travel_mode()
            .map_err(|e| e.to_string())?;
        match val {
            1 => Ok(On),
            0 => Ok(Off),
            v => Err(format!("Unable to parse travel mode '{}'", v)),
        }
    }

    pub fn change_travel_mode(&self, state: OnOffToggleState) -> Result<(), String> {
        use OnOffToggleState::*;
        match state {
            Toggle => self.set_travel_mode(!(self.get_travel_mode()?)),
            other => self.set_travel_mode(other.try_into().unwrap()),
        }
    }

    pub fn print_travel_mode(&self) -> Result<(), String> {
        println!("travel-mode: {}", self.get_travel_mode()?);
        Ok(())
    }

    pub fn await_travel_mode_change(&self) -> Result<(), String> {
        self.misc_interface.receive_travel_mode_changed()
            .map_err(|e| e.to_string())?
            .next();
        Ok(())
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
            Toggle => Err("Attempting to conver 'Toggle' value to OnOffState".to_string()),
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
