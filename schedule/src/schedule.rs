use super::{DisciplineWasFound, Schedule, ScheduleError, ScheduleUnity};
use class::Disciplina;
use stf::{Dia, SigaaTime, Turno};

impl Schedule {
    /// Cria uma nova instância de `Schedule` com uma matriz de `ScheduleUnity` inicializada.
    ///
    /// A matriz tem 8 linhas e 6 colunas, representando diferentes turnos e dias da semana.
    /// Cada `ScheduleUnity` é inicializado com um horário (`SigaaTime`) e nenhuma disciplina.
    ///
    /// # Exemplo
    ///
    /// ```
    /// use schedule::Schedule;
    /// let schedule = Schedule::new();
    /// ```
    pub fn new() -> Schedule {
        Schedule((0..8).map(create_row).collect())
    }

    /// Obtém uma referência mutável para um `ScheduleUnity` específico.
    ///
    /// # Parâmetros
    ///
    /// * `turno_index` - O índice do turno (linha) na matriz.
    /// * `dia_index` - O índice do dia (coluna) na matriz.
    ///
    /// # Retorno
    ///
    /// Retorna uma referência mutável para o `ScheduleUnity` se o índice for válido, caso contrário, retorna `None`.
    fn get_mut(&mut self, turno_index: usize, dia_index: usize) -> Option<&mut ScheduleUnity> {
        self.0
            .get_mut(turno_index)
            .and_then(|row| row.get_mut(dia_index))
    }

    /// Verifica se uma disciplina pode ser inserida sem conflitos.
    ///
    /// # Parâmetros
    ///
    /// * `disciplina` - A disciplina a ser verificada.
    ///
    /// # Retorno
    ///
    /// Retorna `DisciplineWasFound::DisciplineFound` se uma disciplina já estiver ocupando algum dos horários
    /// da disciplina fornecida, caso contrário, retorna `DisciplineWasFound::DisciplineNotFound`.
    pub fn verify_availability(&self, disciplina: &Disciplina) -> DisciplineWasFound {
        for &sigaa_time in &disciplina.sigaa_time {
            let dia_index: usize = sigaa_time.dia.into();
            let turno_index: usize = sigaa_time.turno.into();

            if let Some(schedule_unity) = self.get(turno_index, dia_index) {
                if let Some(disciplina_ocupada) = &schedule_unity.disciplina {
                    return DisciplineWasFound::DisciplineFound(disciplina_ocupada.clone());
                }
            }
        }

        DisciplineWasFound::DisciplineNotFound
    }

    /// Insere uma disciplina no cronograma.
    ///
    /// Se a disciplina não estiver ocupando nenhum horário existente, ela será inserida nos horários correspondentes.
    /// Se houver conflitos com disciplinas existentes, retorna um erro.
    ///
    /// # Parâmetros
    ///
    /// * `disciplina` - A disciplina a ser inserida.
    ///
    /// # Retorno
    ///
    /// Retorna `Ok(())` se a inserção for bem-sucedida, ou um erro do tipo `ScheduleError` se houver conflitos ou problemas.
    pub fn insert(&mut self, disciplina: Disciplina) -> Result<(), ScheduleError> {
        match self.verify_availability(&disciplina) {
            DisciplineWasFound::DisciplineNotFound => {
                for &sigaa_time in &disciplina.sigaa_time {
                    let dia_index: usize = sigaa_time.dia.into();
                    let turno_index: usize = sigaa_time.turno.into();

                    match self.get_mut(turno_index, dia_index) {
                        Some(schedule_unity) => {
                            schedule_unity.disciplina = Some(disciplina.clone())
                        }
                        None => {
                            return Err(ScheduleError::TimeNotFound(SigaaTime::new(
                                dia_index.try_into().unwrap(),
                                turno_index.try_into().unwrap(),
                            )))
                        }
                    }
                }

                Ok(())
            }
            DisciplineWasFound::DisciplineFound(found_discipline) => Err(
                ScheduleError::ConflictingDisciplines(found_discipline, disciplina),
            ),
        }
    }

    /// Obtém uma referência para um `ScheduleUnity` específico.
    ///
    /// # Parâmetros
    ///
    /// * `row` - O índice da linha (turno) na matriz.
    /// * `col` - O índice da coluna (dia) na matriz.
    ///
    /// # Retorno
    ///
    /// Retorna uma referência para o `ScheduleUnity` se o índice for válido, caso contrário, retorna `None`.
    pub fn get(&self, row: usize, col: usize) -> Option<&ScheduleUnity> {
        self.0.get(row)?.get(col)
    }

    /// Obtém uma referência para um `ScheduleUnity` específico.
    ///
    /// # Parâmetros
    ///
    /// * `input` - Uma &str no formato do SigaaTime.
    ///
    ///
    /// # Retorno
    ///
    /// Retorna uma referência para o `ScheduleUnity` se a for válido, caso contrário, retorna `None`.
    pub fn get_from_str(&self, input: &str) -> Option<&ScheduleUnity> {
        let sigaa_time_str: SigaaTime = input.try_into().unwrap();

        let (turno, dia): (usize, usize) = (sigaa_time_str.turno.into(), sigaa_time_str.dia.into());

        self.0.get(turno)?.get(dia)
    }

    /// Obtém as dimensões da matriz de cronograma.
    ///
    /// # Retorno
    ///
    /// Retorna uma tupla contendo o número de linhas e colunas na matriz do cronograma.
    pub fn len(&self) -> (usize, usize) {
        (self.0.len(), self.0[0].len())
    }
}

impl Default for Schedule {
    fn default() -> Self {
        Self::new()
    }
}

fn create_row(row: usize) -> Vec<ScheduleUnity> {
    (0..6)
        .map(|col| (row, col))
        .map(usize_to_turno_dia)
        .map(create_schedule_unity)
        .collect()
}

fn create_schedule_unity((row, col): (Turno, Dia)) -> ScheduleUnity {
    ScheduleUnity::new(SigaaTime::new(col, row), None)
}

fn usize_to_turno_dia((row, col): (usize, usize)) -> (Turno, Dia) {
    (row.try_into().unwrap(), col.try_into().unwrap())
}
