use std::fmt::Display;

use super::{sigaa_time::SigaaTimeErrors, Horario};

impl Display for Horario {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Horario::Primeiro => write!(f, "12"),
            Horario::Segundo => write!(f, "34"),
            Horario::Terceiro => write!(f, "56"),
        }
    }
}

impl TryFrom<usize> for Horario {
    type Error = SigaaTimeErrors;
    fn try_from(value: usize) -> Result<Self, SigaaTimeErrors> {
        match value {
            0 => Ok(Horario::Primeiro),
            1 => Ok(Horario::Segundo),
            2 => Ok(Horario::Terceiro),
            _ => Err(SigaaTimeErrors::InvalidUsizeToHorario),
        }
    }
}

impl TryFrom<&str> for Horario {
    type Error = SigaaTimeErrors;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "12" => Ok(Horario::Primeiro),
            "34" => Ok(Horario::Segundo),
            "56" => Ok(Horario::Terceiro),
            _ => Err(SigaaTimeErrors::InvalidStringToHorario),
        }
    }
}
