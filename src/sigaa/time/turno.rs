use std::fmt::Display;

use super::{SigaaTimeErrors, Turno};

impl Display for Turno {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Turno::Manhã => write!(f, "M"),
            Turno::Tarde => write!(f, "T"),
            Turno::Noite => write!(f, "N"),
        }
    }
}

impl TryFrom<&str> for Turno {
    type Error = SigaaTimeErrors;
    fn try_from(value: &str) -> Result<Self, SigaaTimeErrors> {
        match value {
            "M" => Ok(Turno::Manhã),
            "T" => Ok(Turno::Tarde),
            "N" => Ok(Turno::Noite),
            _ => Err(SigaaTimeErrors::InvalidStringToTurno),
        }
    }
}
