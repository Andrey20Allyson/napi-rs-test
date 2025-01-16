use crate::schedule::schedule_table::{
  DutyRef, ExtraDuty, ExtraScheduleTable, Worker, WorkerRef, WorkerRefArray,
};

pub struct AssignStep {
  pub in_pairs: bool,
  pub full_day: bool,
  pub min: u8,
  pub max: u8,
  pub duty_min_distance: u8,

  current_duty_limit: u8,
}

impl Default for AssignStep {
  fn default() -> Self {
    AssignStep {
      duty_min_distance: 2,
      min: 1,
      max: 1,
      full_day: false,
      in_pairs: true,
      current_duty_limit: 1,
    }
  }
}

pub struct ScheduleAssigner {
  pub step: AssignStep,
}

impl ScheduleAssigner {
  pub fn assign(&mut self, table: &mut ExtraScheduleTable) {
    let mut worker_refs = table.get_worker_ref_array();

    self.assign_array(table, worker_refs);
  }

  pub fn assign_array(&mut self, table: &mut ExtraScheduleTable, worker_refs: WorkerRefArray) {}

  pub fn assign_in_day(&mut self, table: &mut ExtraScheduleTable, worker_refs: WorkerRefArray) {}

  pub fn can_assing(
    &self,
    table: &mut ExtraScheduleTable,
    worker: &Worker,
    duty: &ExtraDuty,
  ) -> bool {
    // [rule set]

    // desactived duty
    if duty.actived == false {
      return false;
    }

    // duty capacity
    if duty.is_full() {
      return false;
    }

    // duty limit
    if duty.workers_len >= self.step.current_duty_limit {
      return false;
    }

    // fem rule
    if worker.gender.is_fem() && duty.is_empty() {
      return false;
    }

    // insp rule
    if worker.grad.is_insp() && duty.insp_count > 0 {
      return false;
    }

    return true;
  }
}
