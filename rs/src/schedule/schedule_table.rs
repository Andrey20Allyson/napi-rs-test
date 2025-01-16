use super::{
  constants::{DUTY_QUANTITY, NUM_OF_DAYS_PER_MONTH, WORKER_LIMIT},
  day_ref::DayRefArray,
  duty::ExtraDuty,
  duty_ref::DutyRef,
  worker::Worker,
  worker_ref::{WorkerRef, WorkerRefArray},
};

#[derive(Clone, Copy)]
pub struct Month {
  pub year: u16,
  pub index: u16,
  num_of_days: u8,
}

impl Month {
  pub fn new(year: u16, index: u16) -> Self {
    Month {
      year,
      index,
      num_of_days: NUM_OF_DAYS_PER_MONTH[index as usize],
    }
  }

  pub fn get_num_of_days(&self) -> u8 {
    self.num_of_days
  }
}

pub struct ExtraScheduleTable {
  pub month: Month,
  pub duties: [ExtraDuty; DUTY_QUANTITY],
  pub workers: [Worker; WORKER_LIMIT],
  pub workers_len: u8,
}

impl ExtraScheduleTable {
  pub fn new(month: Month) -> Self {
    ExtraScheduleTable {
      month,
      duties: [Default::default(); DUTY_QUANTITY],
      workers: [Default::default(); WORKER_LIMIT],
      workers_len: 0,
    }
  }

  pub fn add_worker_to_duty(&mut self, duty_ref: DutyRef, worker_ref: WorkerRef) {
    let worker_grad = self.get_worker(worker_ref).grad;

    let duty = self.get_duty_mut(duty_ref);

    duty.workers[duty.workers_len as usize] = worker_ref;
    duty.workers_len += 1;

    if worker_grad.is_insp() {
      duty.insp_count += 1;
    } else if worker_grad.is_sub() {
      duty.sub_count += 1;
    }
  }

  pub fn get_duty_mut(&mut self, duty_ref: DutyRef) -> &mut ExtraDuty {
    &mut self.duties[duty_ref.get_duty_index()]
  }

  pub fn add_worker(&mut self, worker: Worker) {
    self.workers[self.workers_len as usize] = worker;
    self.workers_len += 1;
  }

  pub fn get_worker(&self, worker_ref: WorkerRef) -> &Worker {
    &self.workers[worker_ref.0 as usize]
  }

  pub fn get_worker_ref_array(&self) -> WorkerRefArray {
    let mut array: [WorkerRef; WORKER_LIMIT] = [Default::default(); WORKER_LIMIT];
    let mut len: usize = 0;

    for i in 0..self.workers_len {
      array[len] = WorkerRef(i);
      len += 1;
    }

    WorkerRefArray::new(array, len)
  }

  pub fn get_day_ref_array(&self) -> DayRefArray {
    let range = 0..self.month.get_num_of_days();

    DayRefArray::from_range(range)
  }
}
