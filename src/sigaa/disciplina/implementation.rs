use std::collections::BTreeSet;

use super::SigaaTime;
use super::{Disciplina, DisciplinaErrors};

impl Disciplina {
    pub fn new(nome: String) -> Disciplina {
        Disciplina {
            sigaa_time: BTreeSet::new(),
            abreviacao: Self::generate_abreviação(nome.clone()),
            nome,
        }
    }

    pub fn new_with_time(nome: String, sigaa_time: BTreeSet<SigaaTime>) -> Disciplina {
        Disciplina {
            sigaa_time,
            abreviacao: Self::generate_abreviação(nome.clone()),
            nome,
        }
    }

    pub fn new_with_time_string(
        nome: String,
        new_time: String,
    ) -> Result<Disciplina, DisciplinaErrors> {
        if !Self::is_formatted(new_time.clone()) {
            return Err(DisciplinaErrors::WrongInputFormat);
        }

        let new_times: BTreeSet<SigaaTime> = Self::valid_string_to_vec_sigaa_times(new_time);

        Ok(Disciplina {
            sigaa_time: new_times,
            abreviacao: Self::generate_abreviação(nome.clone()),
            nome,
        })
    }

    pub fn add_time(&mut self, new_time: SigaaTime) -> Result<(), DisciplinaErrors> {
        self.sigaa_time.insert(new_time);
        Ok(())
    }
}
