mod time;
use disciplina::Disciplina;
use time::{SigaaTime, SigaaTimeErrors};

mod disciplina;

#[derive(Debug, Clone, PartialEq)]
pub struct ScheduleUnity {
    horario: SigaaTime,
    disciplina: Option<Disciplina>,
}

pub struct Schedule(Vec<Vec<ScheduleUnity>>);

#[derive(Debug, PartialEq)]
pub enum ScheduleError {
    ConflictingDisciplines(Disciplina, Disciplina),
    TimeNotFound(SigaaTime),
    SigaaTimeErrors(SigaaTimeErrors),
}

mod schedule;
