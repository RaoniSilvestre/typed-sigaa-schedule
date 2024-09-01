use super::Disciplina;

impl Disciplina {
    pub fn new(nome: String) -> Disciplina {
        Disciplina {
            sigaa_time: Vec::new(),
            abreviacao: generate_abreviação(nome.clone()),
            nome,
        }
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

#[cfg(test)]
mod tests {
    use super::generate_abreviação;

    #[test]
    fn should_generate_a_correct_abrv() {
        let s: String = "Algo Muideto Foda de tlgd".into();
        let abv = generate_abreviação(s);
        assert_eq!(abv, "AMFT")
    }
}
