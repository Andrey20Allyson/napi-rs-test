const {
  ExtraScheduleTable,
  startScheduleAssign,
} = require("./dist/native/my-lib");

const Gender = {
  male: 1,
  fem: 2,
};

const Graduation = {
  insp: 1,
  sub: 2,
  gcm: 3,
};

startScheduleAssign(
  new ExtraScheduleTable({
    month: { year: 2025, index: 0 },
    workers: [
      {
        id: 120,
        gender: Gender.male,
        grad: Graduation.gcm,
      },
    ],
  })
);
