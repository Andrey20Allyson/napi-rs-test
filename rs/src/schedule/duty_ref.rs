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

  pub fn iter(&self) -> DutyRefIter {
    DutyRefIter::new(self)
  }
}

pub struct DutyRefIter<'a> {
  array: &'a DutyRefOfOneDayArray,
  iter_count: usize,
}

impl<'a> DutyRefIter<'a> {
  pub fn new(array: &'a DutyRefOfOneDayArray) -> Self {
    DutyRefIter {
      array,
      iter_count: 0,
    }
  }
}

impl<'a> std::iter::Iterator for DutyRefIter<'a> {
  type Item = DutyRef;

  fn next(&mut self) -> Option<Self::Item> {
    if self.iter_count >= self.array.len {
      return None;
    }

    let duty_ref = self.array.array[self.iter_count];
    self.iter_count += 1;

    Some(duty_ref)
  }
}
