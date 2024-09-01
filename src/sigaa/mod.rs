mod time;
use time::SigaaTime;

#[derive(Debug, Clone, PartialEq)]
pub struct Disciplina {
    nome: String,
    abreviacao: String,
    sigaa_time: Vec<SigaaTime>,
}

mod disciplina;

#[derive(Debug, Clone, PartialEq)]
pub struct ScheduleUnity {
    horario: SigaaTime,
    disciplina: Option<Disciplina>,
}

pub struct Schedule {
    schedule: Vec<Vec<ScheduleUnity>>,
}

mod schedule;
