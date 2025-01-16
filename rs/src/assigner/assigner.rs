use crate::schedule::{
  day_ref::{DayRef, DayRefArray},
  duty_ref::DutyRefIter,
  schedule_table::ExtraScheduleTable,
  worker_ref::{WorkerRef, WorkerRefArray},
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

    let mut day_refs = table.get_day_ref_array();

    let start_duty_limit = self.step.current_duty_limit;

    for limit in self.step.min..=self.step.max {
      self.step.current_duty_limit = limit;

      day_refs.randomize();

      self.assign_in_days(table, &day_refs, &mut worker_refs);
    }

    self.step.current_duty_limit = start_duty_limit;
  }

  pub fn assign_in_days(
    &mut self,
    table: &mut ExtraScheduleTable,
    day_refs: &DayRefArray,
    worker_refs: &mut WorkerRefArray,
  ) {
    for day_ref in day_refs.iter() {
      self.assign_full_day(table, day_ref, worker_refs);
    }
  }

  pub fn assign_full_day(
    &mut self,
    table: &mut ExtraScheduleTable,
    day_ref: DayRef,
    worker_refs: &mut WorkerRefArray,
  ) {
    let duty_refs = day_ref.get_duty_ref_array();

    worker_refs.randomize();

    for worker_ref in worker_refs.iter() {
      let can_assign = self.can_assing(table, worker_ref, duty_refs.iter());
      if can_assign == false {
        continue;
      }

      table.add_worker_to_duties(duty_refs.iter(), worker_ref);
    }
  }

  pub fn can_assing(
    &self,
    table: &mut ExtraScheduleTable,
    worker_ref: WorkerRef,
    duty_refs: DutyRefIter,
  ) -> bool {
    for duty_ref in duty_refs {
      let worker = table.get_worker(worker_ref);
      let duty = table.get_duty(duty_ref);

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
    }

    return true;
  }
}
