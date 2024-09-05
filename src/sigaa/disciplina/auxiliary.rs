use core::panic;
use std::collections::BTreeSet;
use std::ops::Deref;

use super::Disciplina;
use super::DisciplinaErrors;
use super::SigaaTime;
use regex::Regex;

impl Disciplina {
    pub fn generate_abreviação(nome: &str) -> String {
        let mut abreviação = String::new();
        let palavras_excluidas = vec!["de", "em", "da", "do", "das", "dos"];

        for substring in nome.split_whitespace() {
            if !palavras_excluidas.contains(&substring.to_lowercase().as_str()) {
                if let Some(primeira_letra) = substring.chars().next() {
                    abreviação.push(primeira_letra.to_ascii_uppercase())
                }
            }
        }

        abreviação
    }

    pub fn generate_horario_display(&self) -> String {
        let mut output = String::new();
        for elem in self.sigaa_time.iter() {
            output = format!("{}{}", output, elem.dia)
        }

        if let Some(val) = self.sigaa_time.first() {
            output = format!("{}{}", output, val.turno);
        }

        output
    }
    pub fn is_formatted(time: &str) -> bool {
        let regex = Regex::new(r"^(\d{1,5})([MTN])(\d{2,6})$").unwrap();

        if let Some(captura) = regex.captures(&time) {
            let dd = &captura[1];
            let hhhh = &captura[3];

            for elem in dd.chars() {
                if let Some(dd_num) = elem.to_digit(10) {
                    if !(2..=7).contains(&dd_num) {
                        return false;
                    }
                } else {
                    return false;
                }
            }

            if hhhh
                .chars()
                .all(|c| c.is_digit(10) && ('1'..='6').contains(&c))
            {
                return true;
            }
        }

        false
    }

    pub fn valid_string_to_vec_sigaa_times(valid_string: &str) -> BTreeSet<SigaaTime> {
        Self::breakdown(valid_string, &mut BTreeSet::new())
    }

    fn breakdown(value: &str, vec: &mut BTreeSet<SigaaTime>) -> BTreeSet<SigaaTime> {
        let days = Self::extract_day(&value).len();

        match days {
            1 if value.len() == 4 => {
                vec.insert(value.try_into().unwrap());
            }
            1 => Self::hour_breaker(&value, vec),
            _ => Self::day_breaker(days, &value, vec),
        }

        vec.deref().clone()
    }

    fn day_breaker(loops: usize, value: &str, vec: &mut BTreeSet<SigaaTime>) {
        for i in 0..loops {
            let break_value: String = value
                .chars()
                .enumerate()
                .filter(|(index, _)| *index != i)
                .map(|(_, c)| c)
                .collect();
            Self::breakdown(&break_value, vec);
        }
    }

    fn hour_breaker(value: &str, vec: &mut BTreeSet<SigaaTime>) {
        let prefix = &value[..2];
        let first_suffix = &value[2..4];
        let last_suffix = &value[4..];

        let first_part = format!("{}{}", prefix, first_suffix);
        let last_part = format!("{}{}", prefix, last_suffix);

        Self::breakdown(&first_part, vec);
        Self::breakdown(&last_part, vec);
    }

    fn extract_day(texto: &str) -> &str {
        let finder = texto.find(|c| c == 'T' || c == 'M' || c == 'N');

        match finder {
            Some(pos) => texto.split_at(pos).0,
            None => panic!("{:?}: {}", DisciplinaErrors::TurnoNotFounded, texto),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::sigaa::time::SigaaTime;

    use super::Disciplina;

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
}
