const { ExtraScheduleTable } = require("./dist/native/my-lib");

console.log(new ExtraScheduleTable({ month: { year: 4, index: 1 } }).month);
