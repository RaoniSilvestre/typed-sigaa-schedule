use std::fmt::Display;

use super::{HorarioDiurno, HorarioNoturno, SigaaTimeErrors, Turno};

impl Display for Turno {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Turno::Manhã(horario) => match horario {
                HorarioDiurno::Primeiro => write!(f, "M12"),
                HorarioDiurno::Segundo => write!(f, "M34"),
                HorarioDiurno::Terceiro => write!(f, "M56"),
            },
            Turno::Tarde(horario) => match horario {
                HorarioDiurno::Primeiro => write!(f, "T12"),
                HorarioDiurno::Segundo => write!(f, "T34"),
                HorarioDiurno::Terceiro => write!(f, "T56"),
            },
            Turno::Noite(horario) => match horario {
                HorarioNoturno::Primeiro => write!(f, "N12"),
                HorarioNoturno::Segundo => write!(f, "N34"),
            },
        }
    }
}

impl TryFrom<&str> for Turno {
    type Error = SigaaTimeErrors;
    fn try_from(value: &str) -> Result<Self, SigaaTimeErrors> {
        match value {
            "M12" => Ok(Turno::Manhã(HorarioDiurno::Primeiro)),
            "M34" => Ok(Turno::Manhã(HorarioDiurno::Segundo)),
            "M56" => Ok(Turno::Manhã(HorarioDiurno::Terceiro)),
            "T12" => Ok(Turno::Tarde(HorarioDiurno::Primeiro)),
            "T34" => Ok(Turno::Tarde(HorarioDiurno::Segundo)),
            "T56" => Ok(Turno::Tarde(HorarioDiurno::Terceiro)),
            "N12" => Ok(Turno::Noite(HorarioNoturno::Primeiro)),
            "N34" => Ok(Turno::Noite(HorarioNoturno::Segundo)),
            _ => Err(SigaaTimeErrors::InvalidStringToTurno),
        }
    }
}

impl Into<String> for Turno {
    fn into(self) -> String {
        match self {
            Self::Manhã(HorarioDiurno::Primeiro) => "M12",
            Self::Manhã(HorarioDiurno::Segundo) => "M34",
            Self::Manhã(HorarioDiurno::Terceiro) => "M56",
            Self::Tarde(HorarioDiurno::Primeiro) => "T12",
            Self::Tarde(HorarioDiurno::Segundo) => "T34",
            Self::Tarde(HorarioDiurno::Terceiro) => "T56",
            Self::Noite(HorarioNoturno::Primeiro) => "N12",
            Self::Noite(HorarioNoturno::Segundo) => "N34",
        }
        .to_string()
    }
}

impl TryFrom<usize> for Turno {
    type Error = SigaaTimeErrors;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Manhã(HorarioDiurno::Primeiro)),
            1 => Ok(Self::Manhã(HorarioDiurno::Segundo)),
            2 => Ok(Self::Manhã(HorarioDiurno::Terceiro)),
            3 => Ok(Self::Tarde(HorarioDiurno::Primeiro)),
            4 => Ok(Self::Tarde(HorarioDiurno::Segundo)),
            5 => Ok(Self::Tarde(HorarioDiurno::Terceiro)),
            6 => Ok(Self::Noite(HorarioNoturno::Primeiro)),
            7 => Ok(Self::Noite(HorarioNoturno::Segundo)),
            _ => Err(SigaaTimeErrors::InvalidUsizeToHorario),
        }
    }
}

impl Into<usize> for Turno {
    fn into(self) -> usize {
        match self {
            Self::Manhã(HorarioDiurno::Primeiro) => 0,
            Self::Manhã(HorarioDiurno::Segundo) => 1,
            Self::Manhã(HorarioDiurno::Terceiro) => 2,
            Self::Tarde(HorarioDiurno::Primeiro) => 3,
            Self::Tarde(HorarioDiurno::Segundo) => 4,
            Self::Tarde(HorarioDiurno::Terceiro) => 5,
            Self::Noite(HorarioNoturno::Primeiro) => 6,
            Self::Noite(HorarioNoturno::Segundo) => 7,
        }
    }
}
