#[cfg(test)]
mod schedule_tests {
    use class::Disciplina;
    use schedule::{DisciplineWasFound, Schedule, ScheduleUnity};
    use stf::{Dia, HorarioDiurno, SigaaTime, SigaaTimeErrors, Turno};

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

        assert_eq!(schedule.get_from_str("2M12"), Some(&schedule_unity));
        assert_eq!(schedule.get_from_str("3M12"), Some(&schedule_unity_2));

        Ok(())
    }

    #[test]
    fn insert_into_schedule_should_return_ok() {
        let mut schedule = Schedule::new();

        let disciplina_1 = Disciplina::new_stringify("Fundamentos mamáticos da computação I", "246M12").unwrap();

        assert_eq!(schedule.insert(disciplina_1.clone()), Ok(()));
        assert_eq!(schedule.get_from_str("2M12").unwrap().disciplina, Some(disciplina_1.clone()));

        assert_eq!(schedule.get_from_str("4M12").unwrap().disciplina, Some(disciplina_1.clone()));
        assert_eq!(schedule.get_from_str("6M12").unwrap().disciplina, Some(disciplina_1))
    }

    #[test]
    fn verify_availability_should_return_discipline() {
        let mut schedule = Schedule::new();

        let disciplina_1 = Disciplina::new_stringify("Fundamentos mamáticos da computação I", "246M12").unwrap();

        schedule.insert(disciplina_1.clone()).unwrap();

        assert_eq!(schedule.verify_availability(&disciplina_1), DisciplineWasFound::DisciplineFound(disciplina_1.clone()));
    }

    #[test]
    fn insert_and_remove_from_schedule_should_not_have_discipline() {
        let mut schedule = Schedule::new();

        let disciplina_1 = Disciplina::new_stringify("Fundamentos mamáticos da computação I", "246M12").unwrap();

        assert_eq!(schedule.insert(disciplina_1.clone()), Ok(()));
        assert_eq!(schedule.remove(disciplina_1.clone()), Ok(()));
    }
}
