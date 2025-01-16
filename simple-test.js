const {
  ExtraScheduleTable,
  startScheduleAssign,
} = require("./dist/native/my-lib");

startScheduleAssign(
  new ExtraScheduleTable({
    month: { year: 2025, index: 0 },
    workers: [
      {
        id: 1,
      },
    ],
  })
);
