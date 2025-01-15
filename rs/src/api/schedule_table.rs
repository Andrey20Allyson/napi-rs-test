#![deny(clippy::all)]

use napi_derive::napi;

#[derive(Clone)]
#[napi(js_name = "Month", object)]
pub struct JsMonthConfig {
  pub year: i32,
  pub index: i32,
}

#[derive(Clone)]
#[napi(js_name = "WorkerInfoConfig", object)]
pub struct JsWorkerInfoConfig {
  pub id: u32,
}

#[derive(Clone)]
#[napi(js_name = "ExtraScheduleTableCreateConfig", object)]
pub struct JsExtraScheduleTableCreateConfig {
  pub month: JsMonthConfig,
  pub workers: Vec<JsWorkerInfoConfig>,
}

#[napi(js_name = "ExtraScheduleTable")]
pub struct JsExtraScheduleTable {
  pub config: JsExtraScheduleTableCreateConfig,
}

#[napi]
impl JsExtraScheduleTable {
  #[napi(constructor)]
  pub fn new(config: JsExtraScheduleTableCreateConfig) -> napi::Result<Self> {
    let month: i32 = config.month.index;

    println!("month: {}", month);

    Ok(JsExtraScheduleTable { config })
  }
}
