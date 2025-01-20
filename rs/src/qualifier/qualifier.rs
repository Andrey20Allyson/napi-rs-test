use crate::{
  assigner::assigner::{AssignStep, ScheduleAssigner},
  schedule::schedule_table::ExtraScheduleTable,
};

#[derive(Debug, Clone, Copy)]
pub enum InconsistenceWarning {
  IncorrectAllocation,
  DutyMinQuantity,
  GMCOnlyDuty,
}

impl InconsistenceWarning {
  pub fn with_penality(self, penality: u32) -> ScheduleInconsistence {
    ScheduleInconsistence::Warning {
      warning: self,
      penality,
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub enum InconsistenceFailure {
  FemaleOnlyDuty,
}

impl InconsistenceFailure {
  pub fn with_penality(self, penality: u32) -> ScheduleInconsistence {
    ScheduleInconsistence::Failure {
      failure: self,
      penality,
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub enum ScheduleInconsistence {
  Warning {
    warning: InconsistenceWarning,
    penality: u32,
  },
  Failure {
    failure: InconsistenceFailure,
    penality: u32,
  },
}

impl From<InconsistenceFailure> for ScheduleInconsistence {
  fn from(value: InconsistenceFailure) -> Self {
    ScheduleInconsistence::Failure {
      failure: value,
      penality: 0,
    }
  }
}

#[derive(Debug)]
pub struct IntegritySummary {
  pub penality: u32,
  pub has_failure: bool,
}

impl IntegritySummary {
  pub fn new() -> Self {
    IntegritySummary {
      penality: 0,
      has_failure: false,
    }
  }

  pub fn wrost() -> Self {
    IntegritySummary {
      penality: !0,
      has_failure: true,
    }
  }

  pub fn register(&mut self, inconsistence: ScheduleInconsistence) {
    match inconsistence {
      ScheduleInconsistence::Failure {
        failure: _,
        penality,
      } => {
        self.penality += penality;
        self.has_failure = true;
      }
      ScheduleInconsistence::Warning {
        warning: _,
        penality,
      } => {
        self.penality += penality;
      }
    };
  }

  fn is_perfect(&self) -> bool {
    self.penality == 0
  }

  fn is_better_than(&self, other: &Self) -> bool {
    if !self.has_failure && other.has_failure {
      return true;
    }

    if self.has_failure && !other.has_failure {
      return false;
    }

    self.penality < other.penality
  }
}

type IntegrityChecker = fn(&mut IntegritySummary, &ExtraScheduleTable) -> ();

pub struct Qualifier<'a> {
  pub tries_limit: u32,
  pub assign_steps: &'a [AssignStep],
  pub integrity_checkers: &'a [IntegrityChecker],
  assinger: ScheduleAssigner,
}

impl<'a> Qualifier<'a> {
  pub fn new() -> Self {
    Qualifier {
      tries_limit: 1,
      assign_steps: &[],
      integrity_checkers: &[],
      assinger: ScheduleAssigner::new(),
    }
  }

  pub fn set_tries_limit(&mut self, tries_limit: u32) -> &mut Self {
    self.tries_limit = tries_limit;

    self
  }

  pub fn set_assign_configs(&mut self, assign_configs: &'a [AssignStep]) -> &mut Self {
    self.assign_steps = assign_configs;

    self
  }

  pub fn set_integrity_checkers(
    &mut self,
    integrity_checkers: &'a [IntegrityChecker],
  ) -> &mut Self {
    self.integrity_checkers = integrity_checkers;

    self
  }

  pub fn qualify(&mut self, table: &mut ExtraScheduleTable) {
    let mut best_clone = *table;
    let mut best_integrity = IntegritySummary::wrost();

    for _ in 0..self.tries_limit {
      let mut current_clone = *table;

      self.assign(&mut current_clone);

      let current_integrity = self.check_integrity(table);

      // if current_integrity.is_perfect() {
      //   *table = current_clone;
      //   return;
      // }

      if current_integrity.is_better_than(&best_integrity) {
        best_clone = current_clone;
        best_integrity = current_integrity;
      }
    }

    *table = best_clone;
  }

  pub fn check_integrity(&self, table: &ExtraScheduleTable) -> IntegritySummary {
    let mut summary = IntegritySummary::new();

    for checker in self.integrity_checkers.iter() {
      checker(&mut summary, table);
    }

    summary
  }

  pub fn assign(&mut self, table: &mut ExtraScheduleTable) {
    let assigner = &mut self.assinger;

    for step in self.assign_steps.iter() {
      assigner.set_step(step);

      assigner.assign(table);
    }
  }
}
