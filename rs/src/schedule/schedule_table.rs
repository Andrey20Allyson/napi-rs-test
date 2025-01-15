#[derive(Clone, Copy)]
pub struct Month {
  pub year: u16,
  pub index: u16,
}

impl Month {}

#[derive(Clone, Copy)]
pub struct ExtraDuty {
  pub workers: [WorkerRef; 3],
  pub workers_len: u8,
}

impl ExtraDuty {
  fn add_worker_ref(&mut self, wref: WorkerRef) {
    self.workers[self.workers_len as usize] = wref;
    self.workers_len += 1;
  }
}

impl Default for ExtraDuty {
  fn default() -> Self {
    ExtraDuty {
      workers: [WorkerRef(255); 3],
      workers_len: 0,
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
      gender: Gender(0),
      grad: Graduation(0),
    }
  }
}

#[derive(Clone, Copy)]
pub struct WorkerRef(u8);

#[derive(Clone, Copy)]
pub struct Gender(u8);

#[derive(Clone, Copy)]
pub struct Graduation(u8);

const DUTY_QUANTITY: usize = 31 * 4;
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

  pub fn add_worker(&mut self, worker: Worker) {
    self.workers[self.workers_len as usize] = worker;
    self.workers_len += 1;
  }
}
