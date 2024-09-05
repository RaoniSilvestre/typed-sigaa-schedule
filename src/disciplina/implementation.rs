use std::collections::BTreeSet;
use std::fmt::Display;

use super::SigaaTime;
use super::{Disciplina, DisciplinaErrors};

impl Disciplina {
    pub fn new(nome: &str, sigaa_time: BTreeSet<SigaaTime>) -> Disciplina {
        Disciplina {
            sigaa_time,
            abreviacao: Self::generate_abreviação(nome),
            nome: nome.to_string(),
        }
    }

    pub fn new_stringify(nome: &str, new_time: &str) -> Result<Disciplina, DisciplinaErrors> {
        if !Self::is_formatted(new_time) {
            return Err(DisciplinaErrors::NotFormatted);
        }

        let new_times: BTreeSet<SigaaTime> = Self::valid_string_to_vec_sigaa_times(new_time);

        Ok(Disciplina {
            sigaa_time: new_times,
            abreviacao: Self::generate_abreviação(nome),
            nome: nome.to_string(),
        })
    }

    pub fn add_time(&mut self, new_time: SigaaTime) -> Result<(), DisciplinaErrors> {
        match self.sigaa_time.insert(new_time) {
            true => Ok(()),
            false => Err(DisciplinaErrors::TimeAlreadyInserted),
        }
    }
}

impl Display for Disciplina {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} - {} - {}",
            self.nome,
            self.abreviacao,
            self.generate_horario_display()
        )
    }
}
