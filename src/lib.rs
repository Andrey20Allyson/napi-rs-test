#![deny(clippy::all)]

use napi_derive::napi;

#[napi]
pub struct Car {
  pub name: String,
}

#[napi]
impl Car {
  #[napi]
  pub fn get_name(&self) -> &String {
    &self.name
  }

  #[napi]
  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }
}

#[napi]
pub fn times_2_num(num: i32) -> i32 {
  num * 2
}

#[napi]
pub fn fibb(pos: i64) -> i64 {
  if pos > 78 {
    return 0;
  }

  let mut current: i64 = 0;
  let mut prev: i64 = 1;

  for _ in 0..pos {
    let temp = current;
    current = current + prev;
    prev = temp;
  }

  return current;
}

pub fn is_prime(number: i64) -> bool {
  if number <= 1 {
    return false;
  }

  for i in 2..(number / 2) {
    if number % i == 0 {
      return false;
    }
  }

  return true;
}

#[napi]
pub fn highest_prime(upper_limit: i64) -> i64 {
  let mut largest_prime: i64 = -1;

  for i in 2..upper_limit {
    if is_prime(i) {
      largest_prime = i
    }
  }

  return largest_prime;
}

#[napi]
pub fn walk_with_car() -> Car {
  Car {
    name: "Opala".to_string()
  }
}
