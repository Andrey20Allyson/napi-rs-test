use napi_derive::napi;

use crate::schedule::schedule_table::{ExtraScheduleTable, Month};

use super::schedule_table::JsExtraScheduleTable;

#[napi]
pub fn start_schedule_assign(table: &JsExtraScheduleTable) {
  let table = create_schedule_table(table);

  let mut day_refs = table.get_day_ref_array();
  day_refs.randomize();

  println!("{:#?}", day_refs);
}

pub fn create_schedule_table(table: &JsExtraScheduleTable) -> ExtraScheduleTable {
  let month = Month::new(
    table.config.month.year as u16,
    table.config.month.index as u16,
  );

  let table = ExtraScheduleTable::new(month);

  table
}
