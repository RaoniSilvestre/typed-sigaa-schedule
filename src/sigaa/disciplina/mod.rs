use std::collections::BTreeSet;

use super::SigaaTime;

#[derive(Debug, Clone, PartialEq)]
pub struct Disciplina {
    pub nome: String,
    pub abreviacao: String,
    pub sigaa_time: BTreeSet<SigaaTime>,
}

#[derive(Debug)]
pub enum DisciplinaErrors {
    WrongInputFormat,
    TimeAlreadyInserted,
}

mod auxiliary;
mod implementation;
