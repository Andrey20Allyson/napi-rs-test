#![deny(clippy::all)]

use napi_derive::napi;

#[napi]
pub fn times_2(input: u32) -> u32 {
  input * 2
}
