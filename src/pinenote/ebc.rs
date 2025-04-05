use std::fmt::Display;

use zbus::blocking::Connection;

use crate::interfaces::ebc::EbcProxyBlocking;
use super::{OnOffState, OnOffToggleState};


pub struct Ebc<'a> {
    interface: EbcProxyBlocking<'a>,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
#[repr(u8)]
pub enum Waveform {
    A2 = 1,
    DU = 2,
    DU4 = 3,
    GC16 = 4,
    GCC16 = 5,
    GL16 = 6,
    GLR16 = 7,
    GLD16 = 8,
}

impl<'a> Ebc<'a> {
    pub fn new(dbus_connection: &Connection) -> Result<Self, String> {
        let interface = EbcProxyBlocking::new(dbus_connection)
            .map_err(|e| e.to_string())?;
        Ok(Ebc { interface })
    }

    pub fn full_refresh(&self) -> Result<(), String> {
        self.interface.trigger_global_refresh()
            .map_err(|e| e.to_string())
    }

    pub fn set_performance_mode(&self, state: OnOffState) -> Result<(), String> {
        use OnOffState::*;
        let (dclk_val, qop_val) = match state {
            On => (1, 0),
            Off => (0, 1),
        };

        self.interface.set_dclk_select(dclk_val)
            .map_err(|e| e.to_string())?;
        self.interface.request_quality_or_performance_mode(qop_val)
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn get_performance_mode(&self) -> Result<OnOffState, String> {
        let val = self.interface.get_dclk_select()
            .map_err(|e| e.to_string())?;
        use OnOffState::*;
        match val {
            1 => Ok(On),
            0 => Ok(Off),
            v => Err(format!("Unable to parse performance mode '{}'", v)),
        }
    }

    pub fn change_performance_mode(&self, state: OnOffToggleState) -> Result<(), String> {
        use OnOffToggleState::*;
        match state {
            Toggle => self.set_performance_mode(!(self.get_performance_mode()?)),
            other => self.set_performance_mode(other.try_into().unwrap()),
        }
    }

    pub fn print_performance_mode(&self) -> Result<(), String> {
        println!("performance-mode: {}", self.get_performance_mode()?);
        Ok(())
    }

    pub fn await_performance_mode_change(&self) -> Result<(), String> {
        self.interface.receive_dclk_select_changed()
            .map_err(|e| e.to_string())?
            .next();
        Ok(())
    }

    pub fn set_waveform(&self, waveform: Waveform) -> Result<(), String> {
        self.interface.set_default_waveform(waveform as u8)
            .map_err(|e| e.to_string())
    }

    pub fn get_waveform(&self) -> Result<Waveform, String> {
        let val = self.interface.get_default_waveform()
            .map_err(|e| e.to_string())?;
        val.try_into()
    }

    pub fn print_waveform(&self) -> Result<(), String> {
        println!("waveform: {}", self.get_waveform()?);
        Ok(())
    }

    pub fn await_waveform_change(&self) -> Result<(), String> {
        self.interface.receive_waveform_changed()
            .map_err(|e| e.to_string())?
            .next();
        Ok(())
    }
}

impl TryFrom<u8> for Waveform {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use Waveform::*;
        match value {
            1 => Ok(A2),
            2 => Ok(DU),
            3 => Ok(DU4),
            4 => Ok(GC16),
            5 => Ok(GCC16),
            6 => Ok(GL16),
            7 => Ok(GLR16),
            8 => Ok(GLD16),
            v => Err(format!("Unable to parse waveform '{}'", v)),
        }
    }
}

impl Display for Waveform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Waveform::*;
        match self {
            A2 => write!(f, "a2"),
            DU => write!(f, "du"),
            DU4 => write!(f, "du4"),
            GC16 => write!(f, "gc16"),
            GCC16 => write!(f, "gcc16"),
            GL16 => write!(f, "gl16"),
            GLR16 => write!(f, "glr16"),
            GLD16 => write!(f, "gld16"),
        }
    }
}
