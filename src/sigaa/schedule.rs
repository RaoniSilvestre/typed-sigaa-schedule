use super::{time::Turno, Disciplina, Schedule, ScheduleError, ScheduleUnity, SigaaTime};

impl Schedule {
    pub fn new() -> Schedule {
        Schedule(
            (0..8)
                .map(|row| {
                    (0..6)
                        .map(|col| {
                            let dia = col.try_into().unwrap();
                            let turno = row.try_into().unwrap();
                            let sigaa_time = SigaaTime::new(dia, turno);

                            ScheduleUnity::new(sigaa_time, None)
                        })
                        .collect::<Vec<ScheduleUnity>>()
                })
                .collect::<Vec<Vec<ScheduleUnity>>>(),
        )
    }

    fn get_mut(&mut self, turno_index: usize, dia_index: usize) -> Option<&mut ScheduleUnity> {
        self.0
            .get_mut(turno_index)
            .and_then(|row| row.get_mut(dia_index))
    }

    pub fn insert(&mut self, disciplina: Disciplina) -> Result<(), ScheduleError> {
        for &sigaa_time in &disciplina.sigaa_time {
            let dia_index: usize = sigaa_time.dia.into();
            let turno_index: usize = sigaa_time.turno.into();

            if let Some(schedule_unity) = self.get_mut(turno_index, dia_index) {
                if let Some(disciplina_ocupada) = &schedule_unity.disciplina {
                    return Err(ScheduleError::ConflictingDisciplines(
                        disciplina,
                        disciplina_ocupada.clone(),
                    ));
                }
            }
        }

        for &sigaa_time in &disciplina.sigaa_time {
            let dia_index: usize = sigaa_time.dia.into();
            let turno_index: usize = sigaa_time.turno.into();

            match self.get_mut(turno_index, dia_index) {
                Some(schedule_unity) => schedule_unity.disciplina = Some(disciplina.clone()),
                None => {
                    return Err(ScheduleError::TimeNotFound(SigaaTime::new(
                        dia_index.try_into().unwrap(),
                        turno_index.try_into().unwrap(),
                    )))
                }
            }
        }

        Ok(())
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&ScheduleUnity> {
        self.0.get(row)?.get(col)
    }

    pub fn len(&self) -> (usize, usize) {
        (self.0.len(), self.0[0].len())
    }
}

impl ScheduleUnity {
    pub fn new(horario: SigaaTime, disciplina: Option<Disciplina>) -> ScheduleUnity {
        ScheduleUnity {
            horario,
            disciplina,
        }
    }

    pub fn default() -> ScheduleUnity {
        let horario = SigaaTime::new_from_strings("2", "M12").unwrap();
        ScheduleUnity {
            horario,
            disciplina: None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::sigaa::{
        time::{Dia, HorarioDiurno, SigaaTimeErrors, Turno},
        Schedule, ScheduleUnity, SigaaTime,
    };

    #[test]
    fn should_create_a_schedule_unity() {
        let sigaa_time = SigaaTime::new(Dia::Terça, Turno::Tarde(HorarioDiurno::Segundo));

        let schedule_unity = ScheduleUnity::new(sigaa_time, None);

        let sigaa_time_str: String = sigaa_time.to_string();

        assert_eq!(sigaa_time_str, "3T34");
        assert_eq!(schedule_unity.horario, sigaa_time);
        assert_eq!(schedule_unity.disciplina, None);
    }

    #[test]
    fn should_create_a_schedule() -> Result<(), SigaaTimeErrors> {
        let schedule = Schedule::new();

        let sigaa_time = SigaaTime::new_from_strings("2", "M12")?;
        let sigaa_time_2 = SigaaTime::new_from_strings("3", "M12")?;

        let schedule_unity = ScheduleUnity::new(sigaa_time, None);
        let schedule_unity_2 = ScheduleUnity::new(sigaa_time_2, None);

        assert_eq!(schedule.len(), (8, 6));
        assert_eq!(schedule.get(0, 0), Some(&schedule_unity));
        assert_eq!(schedule.get(0, 1), Some(&schedule_unity_2));

        Ok(())
    }
}

#[cfg(test)]
mod test_schedule {
    use super::*;

    #[test]
    fn test_add_disciplina_success() {
        let mut schedule = Schedule::new();

        let disciplina_1 =
            Disciplina::new_stringify("Fundamentos mamáticos da computação I", "246M12").unwrap();

        assert_eq!(schedule.insert(disciplina_1.clone()), Ok(()));
    }
}
