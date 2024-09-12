use class::Disciplina;
use stf::{SigaaTime, SigaaTimeErrors};

/// Representa uma unidade de horário em um cronograma.
///
/// Uma `ScheduleUnity` contém um horário específico (`SigaaTime`) e uma disciplina opcional
/// associada a esse horário.
#[derive(Debug, Clone, PartialEq)]
pub struct ScheduleUnity {
    /// O horário específico para esta unidade.
    pub horario: SigaaTime,
    /// A disciplina associada a este horário, se houver.
    pub disciplina: Option<Disciplina>,
}

/// Representa um cronograma composto por uma matriz de unidades de horário.
///
/// O cronograma é uma coleção bidimensional de `ScheduleUnity`, onde cada `Vec` representa uma linha
/// de horários.
pub struct Schedule(Vec<Vec<ScheduleUnity>>);

/// Erros que podem ocorrer ao trabalhar com cronogramas.
///
/// Estes erros cobrem conflitos entre disciplinas, horários não encontrados e erros relacionados ao `SigaaTime`.
#[derive(Debug, PartialEq)]
pub enum ScheduleError {
    /// Disciplina conflitante com outra disciplina.
    ConflictingDisciplines(Disciplina, Disciplina),
    /// Horário não encontrado no cronograma.
    TimeNotFound(SigaaTime),
    /// Erros associados ao `SigaaTime`.
    SigaaTimeErrors(SigaaTimeErrors),
    /// Disciplina não encontrada para remoção
    DisciplineNotFoundToRemove,
}

/// Resultado de busca de disciplina.
///
/// Indica se a disciplina foi encontrada ou não.
pub enum DisciplineWasFound {
    /// Disciplina encontrada com sucesso.
    DisciplineFound(Disciplina),
    /// Disciplina não encontrada.
    DisciplineNotFound,
}

mod schedule;
mod schedule_unity;
