#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Dia {
    Segunda,
    Terça,
    Quarta,
    Quinta,
    Sexta,
    Sabado,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Turno {
    Manhã,
    Tarde,
    Noite,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Horario {
    Primeiro,
    Segundo,
    Terceiro,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SigaaTime {
    dia: Dia,
    turno: Turno,
    horario: Horario,
}

#[derive(Debug)]
pub enum SigaaTimeErrors {
    TriedToCreateN56,
    InvalidUsizeToDay,
    InvalidStringToDay,
    InvalidUsizeToHorario,
    InvalidStringToTurno,
    InvalidStringToHorario,
}

mod dia;
mod horario;
mod sigaa_time;
mod turno;
