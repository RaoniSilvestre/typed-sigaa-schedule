use std::fmt::Display;

use super::sigaa_time::SigaaTimeErrors;
use super::Dia;

impl Display for Dia {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Dia::Segunda => write!(f, "2"),
            Dia::Terça => write!(f, "3"),
            Dia::Quarta => write!(f, "4"),
            Dia::Quinta => write!(f, "5"),
            Dia::Sexta => write!(f, "6"),
            Dia::Sabado => write!(f, "7"),
        }
    }
}

impl TryFrom<usize> for Dia {
    type Error = SigaaTimeErrors;
    fn try_from(value: usize) -> Result<Self, SigaaTimeErrors> {
        match value {
            0 => Ok(Dia::Segunda),
            1 => Ok(Dia::Terça),
            2 => Ok(Dia::Quarta),
            3 => Ok(Dia::Quinta),
            4 => Ok(Dia::Sexta),
            5 => Ok(Dia::Sabado),
            _ => Err(SigaaTimeErrors::InvalidUsizeToDay),
        }
    }
}

impl TryFrom<&str> for Dia {
    type Error = SigaaTimeErrors;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "2" => Ok(Dia::Segunda),
            "3" => Ok(Dia::Terça),
            "4" => Ok(Dia::Quarta),
            "5" => Ok(Dia::Quinta),
            "6" => Ok(Dia::Sexta),
            "7" => Ok(Dia::Sabado),
            _ => Err(SigaaTimeErrors::InvalidStringToDay),
        }
    }
}
