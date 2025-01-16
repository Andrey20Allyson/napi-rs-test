use super::{
  constants::{U8_NULL, WORKER_LIMIT},
  randomizer,
};

#[derive(Clone, Copy)]
pub struct WorkerRef(pub u8);

impl WorkerRef {
  pub fn is_null(&self) -> bool {
    self.0 == U8_NULL
  }

  pub fn into_index(&self) -> usize {
    if self.is_null() {
      panic!("Null Reference Error")
    }
    self.0 as usize
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

pub struct WorkerRefArray {
  array: [WorkerRef; WORKER_LIMIT],
  len: usize,
}

impl WorkerRefArray {
  pub fn new(array: [WorkerRef; WORKER_LIMIT], len: usize) -> Self {
    WorkerRefArray { array, len }
  }

  pub fn randomize(&mut self) {
    randomizer::randomize_array(&mut self.array, self.len);
  }
}
