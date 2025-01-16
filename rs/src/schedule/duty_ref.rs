use super::constants::{DUTY_PER_DAY, U8_NULL};

#[derive(Clone, Copy)]
pub struct DutyRef {
  pub day: u8,
  pub index: u8,
}

impl DutyRef {
  pub fn is_null(&self) -> bool {
    self.index == U8_NULL
  }

  pub fn get_duty_index(&self) -> usize {
    if self.is_null() {
      panic!("Null Reference Error")
    }
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
pub struct DutyRefOfOneDayArray {
  array: [DutyRef; DUTY_PER_DAY],
  len: usize,
}

impl DutyRefOfOneDayArray {
  pub fn new(array: [DutyRef; DUTY_PER_DAY], len: usize) -> Self {
    DutyRefOfOneDayArray { array, len }
  }
}
