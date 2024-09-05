extern crate sigaa_sched;

#[cfg(test)]
mod auxiliary_disciplina_functions_tests {

    use sigaa_sched::disciplina::Disciplina;
    use sigaa_sched::time::SigaaTime;

    #[test]
    fn should_generate_a_correct_abrv() {
        let abv = Disciplina::generate_abreviação("Algo Muideto Foda de tlgd");
        assert_eq!(abv, "AMFT")
    }

    #[test]
    fn should_return_true_with_formatted_strings() {
        assert!(Disciplina::is_formatted("246T56"));
        assert!(Disciplina::is_formatted("2T12"));
        assert!(Disciplina::is_formatted("5N34"));
        assert!(Disciplina::is_formatted("36M12"));
        assert!(Disciplina::is_formatted("246M56"));
    }

    #[test]
    fn should_return_false_with_non_formatted_strings() {
        assert!(!Disciplina::is_formatted("24X34"));
        assert!(!Disciplina::is_formatted("29M12"));
        assert!(!Disciplina::is_formatted("1234T1234"));
    }

    #[test]
    fn should_create_a_correct_list_of_sigaa_time_days() {
        let sigaa_times = Disciplina::valid_string_to_vec_sigaa_times("246T12");

        let sigaa_time_1 = SigaaTime::new_from_strings("2", "T12").unwrap();
        let sigaa_time_2 = SigaaTime::new_from_strings("4", "T12").unwrap();
        let sigaa_time_3 = SigaaTime::new_from_strings("6", "T12").unwrap();

        let mut sigaa_times_iter = sigaa_times.iter();

        assert_eq!(sigaa_times_iter.next(), Some(&sigaa_time_1));
        assert_eq!(sigaa_times_iter.next(), Some(&sigaa_time_2));
        assert_eq!(sigaa_times_iter.next(), Some(&sigaa_time_3));
    }

    #[test]
    fn should_create_a_correct_list_of_sigaa_time_hour() {
        let sigaa_times = Disciplina::valid_string_to_vec_sigaa_times("2T123456");
        let sigaa_time_1 = SigaaTime::new_from_strings("2", "T12").unwrap();
        let sigaa_time_2 = SigaaTime::new_from_strings("2", "T34").unwrap();
        let sigaa_time_3 = SigaaTime::new_from_strings("2", "T56").unwrap();

        let mut sigaa_times_iter = sigaa_times.iter();

        assert_eq!(sigaa_times_iter.next(), Some(&sigaa_time_1));
        assert_eq!(sigaa_times_iter.next(), Some(&sigaa_time_2));
        assert_eq!(sigaa_times_iter.next(), Some(&sigaa_time_3));
    }

    #[test]
    fn should_create_a_correct_list_of_sigaa_time_days_and_hours() {
        let sigaa_times = Disciplina::valid_string_to_vec_sigaa_times("246T1234");
        let sigaa_time_1 = SigaaTime::new_from_strings("2", "T12").unwrap();
        let sigaa_time_2 = SigaaTime::new_from_strings("2", "T34").unwrap();
        let sigaa_time_3 = SigaaTime::new_from_strings("4", "T12").unwrap();
        let sigaa_time_4 = SigaaTime::new_from_strings("4", "T34").unwrap();
        let sigaa_time_5 = SigaaTime::new_from_strings("6", "T12").unwrap();
        let sigaa_time_6 = SigaaTime::new_from_strings("6", "T34").unwrap();
        let mut sigaa_times_iter = sigaa_times.iter();

        assert_eq!(sigaa_times_iter.next(), Some(&sigaa_time_1));
        assert_eq!(sigaa_times_iter.next(), Some(&sigaa_time_2));
        assert_eq!(sigaa_times_iter.next(), Some(&sigaa_time_3));
        assert_eq!(sigaa_times_iter.next(), Some(&sigaa_time_4));
        assert_eq!(sigaa_times_iter.next(), Some(&sigaa_time_5));
        assert_eq!(sigaa_times_iter.next(), Some(&sigaa_time_6));
    }

    #[test]
    fn should_generate_a_correct_sigaa_time_display() {
        let disciplina = Disciplina::new_stringify("fun mat comp", "246T12").unwrap();
        assert_eq!(disciplina.generate_horario_display(), "246T12");
    }

    #[test]
    fn should_generate_a_correct_horario_display() {
        let dis =
            Disciplina::new_stringify("Fundamentos Matemáticos da Computação", "246T12").unwrap();
        let dis_2 = Disciplina::new_stringify("a", "24M1234").unwrap();

        assert_eq!(dis.generate_horario_display(), "246T12");
        assert_eq!(dis_2.generate_horario_display(), "24M1234");
    }
}
