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
pub fn times_2(mut input: Vec<u32>) -> Vec<u32> {
  for i in 0..input.len() {
    input[i] = input[i] * 2;
  }

  return input;
}

#[napi]
pub fn walk_with_car() -> Car {
  Car {
    name: "Opala".to_string()
  }
}
