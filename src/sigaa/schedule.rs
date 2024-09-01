use super::{Disciplina, Schedule, ScheduleUnity, SigaaTime, Turno};

impl Schedule {
    pub fn new() -> Schedule {
        let mut vector_row: Vec<Vec<ScheduleUnity>> = Vec::new();

        for _ in 0..8 {
            vector_row.push(vec![ScheduleUnity::default(); 6]);
        }

        for row in 0..3 {
            for col in 0..6 {
                let dia = col.try_into().unwrap();
                let turno = Turno::Manhã;
                let horario = row.try_into().unwrap();

                let sigaa_time = SigaaTime::new(dia, turno, horario).unwrap();

                vector_row[row][col] = ScheduleUnity::new(sigaa_time, None)
            }
        }

        for row in 3..6 {
            for col in 0..6 {
                let dia = col.try_into().unwrap();
                let turno = Turno::Tarde;
                let horario = (row % 3).try_into().unwrap();

                let sigaa_time = SigaaTime::new(dia, turno, horario).unwrap();

                vector_row[row][col] = ScheduleUnity::new(sigaa_time, None)
            }
        }

        for row in 6..8 {
            for col in 0..6 {
                let dia = col.try_into().unwrap();
                let turno = Turno::Noite;
                let horario = (row % 3).try_into().unwrap();

                let sigaa_time = SigaaTime::new(dia, turno, horario).unwrap();

                vector_row[row][col] = ScheduleUnity::new(sigaa_time, None)
            }
        }

        Schedule {
            schedule: vector_row,
        }
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
        let horario = SigaaTime::new_from_strings("2", "M", "12").unwrap();
        ScheduleUnity {
            horario,
            disciplina: None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::sigaa::{
        sigaa_time::{SigaaTime, SigaaTimeErrors},
        Dia, Horario, Schedule, ScheduleUnity, Turno,
    };

    #[test]
    fn should_create_a_schedule_unity() {
        let sigaa_time = SigaaTime::new(Dia::Terça, Turno::Tarde, Horario::Segundo).unwrap();

        let schedule_unity = ScheduleUnity::new(sigaa_time, None);

        let sigaa_time_str: String = sigaa_time.to_string();

        assert_eq!(sigaa_time_str, "3T34");
        assert_eq!(schedule_unity.horario, sigaa_time);
        assert_eq!(schedule_unity.disciplina, None);
    }

    #[test]
    fn should_create_a_schedule() -> Result<(), SigaaTimeErrors> {
        let schedule = Schedule::new();

        let sigaa_time = SigaaTime::new_from_strings("2", "M", "12")?;
        let sigaa_time_2 = SigaaTime::new_from_strings("3", "M", "12")?;

        let schedule_unity = ScheduleUnity::new(sigaa_time, None);
        let schedule_unity_2 = ScheduleUnity::new(sigaa_time_2, None);

        assert_eq!(schedule.schedule.len(), 8);
        assert_eq!(schedule.schedule[0][0], schedule_unity);
        assert_eq!(schedule.schedule[0][1], schedule_unity_2);

        Ok(())
    }
}
