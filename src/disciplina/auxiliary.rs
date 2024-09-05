use core::panic;
use std::collections::BTreeSet;
use std::ops::Deref;

use crate::time::Dia;
use crate::time::Turno;

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
        let mut dias: BTreeSet<Dia> = BTreeSet::new();
        let mut turnos: BTreeSet<Turno> = BTreeSet::new();

        for elem in self.sigaa_time.iter() {
            dias.insert(elem.dia);
            turnos.insert(elem.turno);
        }

        for dia in dias.iter() {
            output = format!("{}{}", output, dia);
        }

        for (i, turno) in turnos.iter().enumerate() {
            match i {
                0 => output = format!("{}{}", output, turno),
                _ => {
                    output = format!(
                        "{}{}",
                        output,
                        <Turno as Into<String>>::into(*turno)[1..].to_string()
                    )
                }
            }
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
