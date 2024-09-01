use std::fmt::Display;

use super::{Dia, Horario, SigaaTime, SigaaTimeErrors, Turno};

impl SigaaTime {
    pub fn new(dia: Dia, turno: Turno, horario: Horario) -> Result<SigaaTime, SigaaTimeErrors> {
        if turno == Turno::Noite && horario == Horario::Terceiro {
            return Err(SigaaTimeErrors::TriedToCreateN56);
        }

        Ok(SigaaTime {
            dia,
            turno,
            horario,
        })
    }

    pub fn new_from_strings(
        dia_str: &str,
        turno_str: &str,
        horario_str: &str,
    ) -> Result<SigaaTime, SigaaTimeErrors> {
        let dia: Dia = dia_str.try_into()?;
        let turno: Turno = turno_str.try_into()?;
        let horario: Horario = horario_str.try_into()?;

        Ok(SigaaTime::new(dia, turno, horario)?)
    }
}

impl Display for SigaaTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.dia, self.turno, self.horario)
    }
}
