extern crate sigaa_sched;

#[cfg(test)]
mod schedule_tests {
    use sigaa_sched::disciplina::Disciplina;
    use sigaa_sched::time::{Dia, HorarioDiurno, SigaaTime, SigaaTimeErrors, Turno};
    use sigaa_sched::{Schedule, ScheduleUnity};

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

    #[test]
    fn insert_into_schedule_should_return_ok() {
        let mut schedule = Schedule::new();

        let disciplina_1 =
            Disciplina::new_stringify("Fundamentos mamáticos da computação I", "246M12").unwrap();

        assert_eq!(schedule.insert(disciplina_1.clone()), Ok(()));
    }
}
