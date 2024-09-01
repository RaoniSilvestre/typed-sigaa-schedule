use core::panic;
use std::collections::BTreeSet;
use std::ops::Deref;

use super::Disciplina;
use super::DisciplinaErrors;
use super::SigaaTime;
use regex::Regex;

impl Disciplina {
    pub fn generate_abreviação(nome: String) -> String {
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
    pub fn is_formatted(time: String) -> bool {
        let regex = Regex::new(r"^(\d{1,3})([MTN])(\d{2,4})$").unwrap();

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

    pub fn valid_string_to_vec_sigaa_times(valid_string: String) -> BTreeSet<SigaaTime> {
        Self::breakdown(valid_string, &mut BTreeSet::new())
    }

    fn breakdown(value: String, vec: &mut BTreeSet<SigaaTime>) -> BTreeSet<SigaaTime> {
        let days = Self::extract_day(&value);
        match value.len() {
            4 => {
                vec.insert(value.as_str().try_into().unwrap());
            }
            5 => Self::day_breaker(1, &value, vec),
            6 if days.len() == 3 => Self::day_breaker(2, &value, vec),
            6 if days.len() == 1 => Self::hour_breaker(&value, vec),
            7 => Self::day_breaker(1, &value, vec),
            8 => Self::day_breaker(2, &value, vec),
            _ => panic!("{:?}", DisciplinaErrors::WrongInputFormat),
        }

        vec.deref().clone()
    }

    fn day_breaker(loops: usize, value: &String, vec: &mut BTreeSet<SigaaTime>) {
        for i in 0..=loops {
            let break_value: String = value
                .chars()
                .enumerate()
                .filter(|(index, _)| *index != i)
                .map(|(_, c)| c)
                .collect();
            Self::breakdown(break_value, vec);
        }
    }

    fn hour_breaker(value: &String, vec: &mut BTreeSet<SigaaTime>) {
        if value.len() != 6 {
            panic!("{:?}", DisciplinaErrors::WrongInputFormat);
        }

        let prefix = &value[..2];
        let first_suffix = &value[2..4];
        let last_suffix = &value[4..6];

        let first_part = format!("{}{}", prefix, first_suffix);
        let last_part = format!("{}{}", prefix, last_suffix);

        Self::breakdown(first_part, vec);
        Self::breakdown(last_part, vec);
    }

    fn extract_day(texto: &str) -> &str {
        let finder = texto.find(|c| c == 'T' || c == 'M' || c == 'N');

        match finder {
            Some(pos) => texto.split_at(pos).0,
            None => panic!("{:?}", DisciplinaErrors::WrongInputFormat),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sigaa::time::SigaaTime;

    use super::Disciplina;

    #[test]
    fn should_generate_a_correct_abrv() {
        let s: String = "Algo Muideto Foda de tlgd".into();
        let abv = Disciplina::generate_abreviação(s);
        assert_eq!(abv, "AMFT")
    }

    #[test]
    fn should_return_true_with_formatted_strings() {
        assert!(Disciplina::is_formatted("246T56".to_string()));
        assert!(Disciplina::is_formatted("2T12".to_string()));
        assert!(Disciplina::is_formatted("5N34".to_string()));
        assert!(Disciplina::is_formatted("36M12".to_string()));
        assert!(Disciplina::is_formatted("246M56".to_string()));
    }

    #[test]
    fn should_return_false_with_non_formatted_strings() {
        assert!(!Disciplina::is_formatted("24X34".to_string()));
        assert!(!Disciplina::is_formatted("29M12".to_string()));
        assert!(!Disciplina::is_formatted("1234T1234".to_string()));
    }

    #[test]
    fn should_create_a_correct_list_of_sigaa_times() {
        let time = "246T12".to_string();
        let sigaa_times = Disciplina::valid_string_to_vec_sigaa_times(time);
        let sigaa_time_1 = SigaaTime::new_from_strings("2", "T", "12").unwrap();
        let sigaa_time_2 = SigaaTime::new_from_strings("4", "T", "12").unwrap();
        let sigaa_time_3 = SigaaTime::new_from_strings("6", "T", "12").unwrap();

        let mut sigaa_times_iter = sigaa_times.iter();

        if let Some(val) = sigaa_times_iter.next() {
            assert!(*val == sigaa_time_1);
        }

        if let Some(val) = sigaa_times_iter.next() {
            assert_eq!(*val, sigaa_time_2);
        }

        if let Some(val) = sigaa_times_iter.next() {
            assert_eq!(*val, sigaa_time_3);
        }
    }
}
