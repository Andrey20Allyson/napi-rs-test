use crate::schedule::{
  constants::week_days,
  day_ref::{DayRef, DayRefArray},
  duty_ref::{DutyRef, RefIterable, RefIterator},
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
      max: 2,
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
  pub fn new() -> Self {
    ScheduleAssigner {
      step: Default::default(),
    }
  }

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
      if self.step.full_day {
        self.assign_full_day(table, day_ref, worker_refs);
        continue;
      }

      if self.step.in_pairs {
        self.assign_in_pairs(table, day_ref, worker_refs);
        continue;
      }
    }
  }

  pub fn assign_full_day(
    &mut self,
    table: &mut ExtraScheduleTable,
    day_ref: DayRef,
    worker_refs: &mut WorkerRefArray,
  ) {
    let duty_refs = day_ref.get_duty_ref_pairs().joined();

    worker_refs.randomize();

    for worker_ref in worker_refs.iter() {
      self.try_assign(table, worker_ref, &duty_refs);
    }
  }

  pub fn assign_in_pairs(
    &mut self,
    table: &mut ExtraScheduleTable,
    day_ref: DayRef,
    worker_refs: &mut WorkerRefArray,
  ) {
    let mut duty_ref_pairs = day_ref.get_duty_ref_pairs();

    let is_monday = table.month.week_day_of(day_ref.get_index()) == week_days::MONDAY;

    if is_monday {
      duty_ref_pairs.randomize();
    }

    worker_refs.randomize();

    let first_pair = duty_ref_pairs.get_first_pair();

    for worker_ref in worker_refs.iter() {
      self.try_assign(table, worker_ref, &first_pair);
    }

    worker_refs.randomize();

    let second_pair = duty_ref_pairs.get_second_pair();

    for worker_ref in worker_refs.iter() {
      self.try_assign(table, worker_ref, &second_pair);
    }
  }

  pub fn assign_in_day(
    &mut self,
    table: &mut ExtraScheduleTable,
    day_ref: DayRef,
    worker_refs: &mut WorkerRefArray,
  ) {
    let duty_refs = day_ref.get_duty_ref_array();

    for duty_ref in duty_refs.iter() {
      let iterable_duty_ref = duty_ref.into_iterable();

      for worker_ref in worker_refs.iter() {
        self.try_assign(table, worker_ref, &iterable_duty_ref);
      }
    }
  }

  pub fn try_assign(
    &self,
    table: &mut ExtraScheduleTable,
    worker_ref: WorkerRef,
    duty_refs: &impl RefIterable<DutyRef>,
  ) -> bool {
    if self.can_assing(table, duty_refs.iter(), worker_ref) {
      return false;
    }

    table.add_worker_to_duties(duty_refs.iter(), worker_ref);

    return true;
  }

  pub fn can_assing(
    &self,
    table: &mut ExtraScheduleTable,
    duty_refs: RefIterator<DutyRef>,
    worker_ref: WorkerRef,
  ) -> bool {
    for duty_ref in duty_refs {
      let worker = table.get_worker(worker_ref);
      let worker_assigment_info = table.get_worker_assigment_info(worker_ref);
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

      // worker limit
      if worker_assigment_info.assigment_count >= worker.assign_limit {
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
