use super::{time::SigaaTime, Disciplina};
use regex::{self, Regex};

#[derive(Debug)]
pub enum DisciplinaErrors {
    WrongInputFormat,
    TimeAlreadyInserted,
}

impl Disciplina {
    pub fn new(nome: String) -> Disciplina {
        Disciplina {
            sigaa_time: Vec::new(),
            abreviacao: generate_abreviação(nome.clone()),
            nome,
        }
    }

    pub fn new_with_time(nome: String, sigaa_time: Vec<SigaaTime>) -> Disciplina {
        Disciplina {
            sigaa_time,
            abreviacao: generate_abreviação(nome.clone()),
            nome,
        }
    }

    pub fn new_with_time_string(
        nome: String,
        new_time: String,
    ) -> Result<Disciplina, DisciplinaErrors> {
        if !is_formatted(new_time.clone()) {
            return Err(DisciplinaErrors::WrongInputFormat);
        }

        let new_times: Vec<SigaaTime> = valid_string_to_vec_sigaa_times(new_time);

        Ok(Disciplina {
            sigaa_time: new_times,
            abreviacao: generate_abreviação(nome.clone()),
            nome,
        })
    }

    pub fn add_time(&mut self, new_time: SigaaTime) -> Result<(), DisciplinaErrors> {
        if self.sigaa_time.contains(&new_time) {
            return Err(DisciplinaErrors::TimeAlreadyInserted);
        }
        self.sigaa_time.push(new_time);
        Ok(())
    }
}

fn generate_abreviação(nome: String) -> String {
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

fn is_formatted(time: String) -> bool {
    let regex = Regex::new(r"^(\d{1,2})([MTN])(\d{2,4})$").unwrap();

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

fn valid_string_to_vec_sigaa_times(valid_string: String) -> Vec<SigaaTime> {
    breakdown(valid_string, &mut Vec::new())
}

fn breakdown(value: String, vec: &mut Vec<SigaaTime>) -> Vec<SigaaTime> {
    match value.len() {
        4 => vec.push(value.as_str().try_into().unwrap()),
        5 => down_breaker(1, &value, vec),
        // Tratar caso especial do 6
        // No 6 é o momento q eu decidi que vai estar algo como 2T1234 e eu vou quebrar o 12 e o 34
        6 => down_breaker(2, &value, vec),
        7 => down_breaker(1, &value, vec),
        8 => down_breaker(2, &value, vec),
        _ => panic!("{:?}", DisciplinaErrors::WrongInputFormat),
    }

    vec.to_vec()
}

fn down_breaker(loops: usize, value: &String, vec: &mut Vec<SigaaTime>) {
    for i in 0..=loops {
        let break_value: String = value
            .chars()
            .enumerate()
            .filter(|(index, _)| *index != i)
            .map(|(_, c)| c)
            .collect();
        breakdown(break_value, vec);
    }
}

#[cfg(test)]
mod tests {
    use crate::sigaa::disciplina::is_formatted;

    use super::generate_abreviação;

    #[test]
    fn should_generate_a_correct_abrv() {
        let s: String = "Algo Muideto Foda de tlgd".into();
        let abv = generate_abreviação(s);
        assert_eq!(abv, "AMFT")
    }

    #[test]
    fn should_return_true_with_formatted_strings() {
        assert!(is_formatted("24T56".to_string()));
        assert!(is_formatted("2T12".to_string()));
        assert!(is_formatted("5N34".to_string()));
        assert!(is_formatted("36M12".to_string()));
    }
}
