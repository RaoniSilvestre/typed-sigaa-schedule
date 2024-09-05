#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    Manhã(HorarioDiurno),
    Tarde(HorarioDiurno),
    Noite(HorarioNoturno),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HorarioDiurno {
    Primeiro,
    Segundo,
    Terceiro,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HorarioNoturno {
    Primeiro,
    Segundo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SigaaTime {
    pub dia: Dia,
    pub turno: Turno,
}

#[derive(Debug, PartialEq)]
pub enum SigaaTimeErrors {
    InvalidUsizeToDay,
    InvalidStringToDay,
    InvalidUsizeToHorario,
    InvalidStringToTurno,
    InvalidStringToSigaaTime,
    InvalidStringToHorario,
}

mod dia;
mod sigaa_time;
mod turno;
