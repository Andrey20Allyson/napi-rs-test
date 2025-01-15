use crate::schedule::schedule_table::ExtraScheduleTable;

pub struct AssignStep {
  pub in_pairs: bool,
  pub full_day: bool,
  pub min: u8,
  pub max: u8,
  pub duty_min_distance: u8,
}

impl Default for AssignStep {
  fn default() -> Self {
    AssignStep {
      duty_min_distance: 2,
      min: 1,
      max: 1,
      full_day: false,
      in_pairs: true,
    }
  }
}

pub struct ScheduleAssigner {
  pub step: AssignStep,
}

impl ScheduleAssigner {
  pub fn assign(&self, table: &mut ExtraScheduleTable) {}
}
