use disciplina::Disciplina;
use time::{SigaaTime, SigaaTimeErrors};

#[derive(Debug, Clone, PartialEq)]
pub struct ScheduleUnity {
    pub horario: SigaaTime,
    pub disciplina: Option<Disciplina>,
}

pub struct Schedule(Vec<Vec<ScheduleUnity>>);

#[derive(Debug, PartialEq)]
pub enum ScheduleError {
    ConflictingDisciplines(Disciplina, Disciplina),
    TimeNotFound(SigaaTime),
    SigaaTimeErrors(SigaaTimeErrors),
}

pub enum DisciplineWasFound {
    DisciplineFound(Disciplina),
    DisciplineNotFound,
}

pub mod disciplina;
pub mod schedule;
pub mod time;
