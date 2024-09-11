use std::collections::BTreeSet;

use stf::SigaaTime;

// Representa uma disciplina no sistema.
///
/// Cada disciplina tem um nome, uma abreviação e um conjunto de horários (`SigaaTime`) associados.
#[derive(Debug, Clone, PartialEq)]
pub struct Disciplina {
    /// Nome da disciplina.
    pub nome: String,
    /// Abreviação da disciplina.
    pub abreviacao: String,
    /// Horários associados a esta disciplina.
    pub sigaa_time: BTreeSet<SigaaTime>,
}

/// Erros que podem ocorrer ao lidar com disciplinas.
///
/// Estes erros cobrem formatos de entrada incorretos, horários já inseridos, e outros problemas relacionados
/// à disciplina.
#[derive(Debug)]
pub enum DisciplinaErrors {
    /// Horário já inserido para a disciplina.
    TimeAlreadyInserted,
    /// String de horário não está no formato correto ([2..7][M|T|N][1..6]).
    TimeNotFormatted,
}

mod auxiliary;
mod implementation;
