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
  let mut current: i64 = 0;
  let mut prev: i64 = 1;

  for _ in 0..pos {
    let temp = current;
    current = current + prev;
    prev = temp;
  }

  return current;
}

#[napi]
pub fn walk_with_car() -> Car {
  Car {
    name: "Opala".to_string()
  }
}
