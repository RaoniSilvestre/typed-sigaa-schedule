use sigaa_time::SigaaTime;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Dia {
    Segunda,
    Terça,
    Quarta,
    Quinta,
    Sexta,
    Sabado,
}

mod dia;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Turno {
    Manhã,
    Tarde,
    Noite,
}

mod turno;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Horario {
    Primeiro,
    Segundo,
    Terceiro,
}

mod horario;

mod sigaa_time;

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
