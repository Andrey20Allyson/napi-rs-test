use napi_derive::napi;

use crate::schedule::{
  schedule_table::{ExtraScheduleTable, Month},
  worker::{Gender, Graduation, Worker},
};

use super::schedule_table::{JsExtraScheduleTable, JsExtraScheduleTableCreateConfig};

#[napi]
pub fn start_schedule_assign(table: &JsExtraScheduleTable) {
  let table = create_schedule_table(&table.config);

  let mut day_refs = table.get_day_ref_array();
  day_refs.randomize();

  println!("{:#?}", day_refs);
}

pub fn create_schedule_table(config: &JsExtraScheduleTableCreateConfig) -> ExtraScheduleTable {
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
