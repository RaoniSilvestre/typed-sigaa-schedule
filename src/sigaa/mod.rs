mod time;
use disciplina::Disciplina;
use time::SigaaTime;

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
