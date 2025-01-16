use napi_derive::napi;

use crate::schedule::schedule_table::{ExtraScheduleTable, Month};

use super::schedule_table::JsExtraScheduleTable;

#[napi]
pub fn start_schedule_assign(table: &JsExtraScheduleTable) {
  let mut table = create_schedule_table(table);

  println!("{:#?}", table.get_days_range());
  println!("{:#?}", table.get_days_range());
}

pub fn create_schedule_table(table: &JsExtraScheduleTable) -> ExtraScheduleTable {
  let month = Month::new(
    table.config.month.year as u16,
    table.config.month.index as u16,
  );

  let table = ExtraScheduleTable::new(month);

  table
}
