use super::randomizer;

#[derive(Clone, Copy)]
pub struct Month {
  pub year: u16,
  pub index: u16,
  num_of_days_cache: u8,
}

pub const U8_NULL: u8 = 255;

const NUM_OF_DAYS_PER_MONTH: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

impl Month {
  pub fn new(year: u16, index: u16) -> Self {
    Month {
      year,
      index,
      num_of_days_cache: U8_NULL,
    }
  }

  pub fn get_num_of_days(&mut self) -> u8 {
    if self.num_of_days_cache == U8_NULL {
      let num_of_days = NUM_OF_DAYS_PER_MONTH[self.index as usize];
      self.num_of_days_cache = num_of_days;
    }

    self.num_of_days_cache
  }
}

const WORKER_PER_DUTY: usize = 3;

#[derive(Clone, Copy)]
pub struct ExtraDuty {
  pub workers: [WorkerRef; WORKER_PER_DUTY],
  pub workers_len: u8,
  pub insp_count: u8,
  pub sub_count: u8,
  pub actived: bool,
}

impl ExtraDuty {
  pub fn has(&self, wref: WorkerRef) -> bool {
    if self.workers_len == 0 {
      return false;
    }

    let wref_1 = self.workers[0];

    if wref == wref_1 {
      return true;
    }

    if self.workers_len == 1 {
      return false;
    }

    let wref_2 = self.workers[1];

    if wref == wref_2 {
      return true;
    }

    if self.workers_len == 2 {
      return false;
    }

    let wref_3 = self.workers[2];

    if wref == wref_3 {
      return true;
    }

    return false;
  }

  pub fn has_insp_or_sub(&self) -> bool {
    self.insp_count > 0 || self.sub_count > 0
  }

  pub fn is_empty(&self) -> bool {
    self.workers_len == 0
  }

  pub fn is_full(&self) -> bool {
    self.workers_len == WORKER_PER_DUTY as u8
  }
}

impl Default for ExtraDuty {
  fn default() -> Self {
    ExtraDuty {
      workers: [Default::default(); WORKER_PER_DUTY],
      workers_len: 0,
      insp_count: 0,
      sub_count: 0,
      actived: true,
    }
  }
}

#[derive(Clone, Copy)]
pub struct DutyRef {
  pub day: u8,
  pub index: u8,
}

impl DutyRef {
  pub fn get_duty_index(&self) -> usize {
    (self.day * 4 + self.index) as usize
  }
}

impl Default for DutyRef {
  fn default() -> Self {
    DutyRef {
      day: U8_NULL,
      index: U8_NULL,
    }
  }
}

#[derive(Clone, Copy)]
pub struct Worker {
  pub id: u32,
  pub gender: Gender,
  pub grad: Graduation,
}

impl Default for Worker {
  fn default() -> Self {
    Worker {
      id: 0,
      gender: Gender(U8_NULL),
      grad: Graduation(U8_NULL),
    }
  }
}

#[derive(Clone, Copy)]
pub struct WorkerRef(u8);

impl WorkerRef {
  pub fn is_null(&self) -> bool {
    self.0 == U8_NULL
  }
}

impl Default for WorkerRef {
  fn default() -> Self {
    WorkerRef(U8_NULL)
  }
}

impl PartialEq for WorkerRef {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}

#[derive(Clone, Copy)]
pub struct Gender(u8);

impl Gender {
  pub fn is_male(&self) -> bool {
    self.0 == 1
  }

  pub fn is_fem(&self) -> bool {
    self.0 == 2
  }
}

#[derive(Clone, Copy)]
pub struct Graduation(u8);

impl Graduation {
  pub fn is_insp(&self) -> bool {
    self.0 == 1
  }

  pub fn is_sub(&self) -> bool {
    self.0 == 2
  }

  pub fn is_gcm(&self) -> bool {
    self.0 == 3
  }
}

const DUTY_PER_DAY: usize = 4;
const DUTY_QUANTITY: usize = 32 * DUTY_PER_DAY;
const WORKER_LIMIT: usize = 34;

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

    WorkerRefArray { array, len }
  }

  pub fn get_days_range(&mut self) -> std::ops::Range<u8> {
    0..self.month.get_num_of_days()
  }
}

pub struct WorkerRefArray {
  array: [WorkerRef; WORKER_LIMIT],
  len: usize,
}

impl WorkerRefArray {
  pub fn randomize(&mut self) {
    randomizer::randomize_array(&mut self.array, self.len);
  }
}
