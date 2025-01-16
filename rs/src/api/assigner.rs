use napi_derive::napi;

use crate::{
  assigner::assigner::ScheduleAssigner,
  schedule::{
    schedule_table::{ExtraScheduleTable, Month},
    worker::{Gender, Graduation, Worker},
  },
};

use super::schedule_table::{JsExtraScheduleTable, JsExtraScheduleTableCreateConfig};

#[napi(js_name = "ScheduleAssignState", object)]
pub struct JsScheduleAssignState {
  pub worker_id: u32,
  pub duty_index: u8,
  pub day_index: u8,
}

#[napi(js_name = "ExtraScheduleTableOutputConfig", object)]
pub struct JsExtraScheduleTableOutputConfig {
  pub assign_state: Vec<JsScheduleAssignState>,
}

#[napi]
pub fn start_schedule_assign(table: &JsExtraScheduleTable) -> JsExtraScheduleTableOutputConfig {
  let mut table = create_schedule_table(&table.config);

  let mut assigner = ScheduleAssigner::new();

  assigner.assign(&mut table);

  create_output_config(&table)
}

fn create_schedule_table(config: &JsExtraScheduleTableCreateConfig) -> ExtraScheduleTable {
  let month = Month::new(config.month.year as u16, config.month.index as u16);

  let mut table = ExtraScheduleTable::new(month);

  for worker_config in config.workers.iter() {
    table.add_worker(Worker {
      id: worker_config.id,
      gender: Gender(worker_config.gender),
      grad: Graduation(worker_config.grad),
    });
  }

  table
}

fn create_output_config(table: &ExtraScheduleTable) -> JsExtraScheduleTableOutputConfig {
  let mut vec: Vec<JsScheduleAssignState> = Vec::new();

  for day_ref in table.get_day_ref_array().iter() {
    for duty_ref in day_ref.get_duty_ref_array().iter() {
      let duty = table.get_duty(duty_ref);

      for worker_ref in duty.iter_worker_refs() {
        let worker = table.get_worker(worker_ref);

        let state = JsScheduleAssignState {
          duty_index: duty_ref.index,
          day_index: duty_ref.day,
          worker_id: worker.id,
        };

        vec.push(state);
      }
    }
  }

  JsExtraScheduleTableOutputConfig { assign_state: vec }
}
