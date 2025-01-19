const { startScheduleAssign } = require("./dist/native/my-lib");

const Gender = {
  male: 1,
  fem: 2,
};

const Graduation = {
  insp: 1,
  sub: 2,
  gcm: 3,
};

const lbl = "Assignment";

console.time(lbl);
let out = startScheduleAssign({
  month: { year: 2025, index: 0 },
  workers: [
    {
      id: 120,
      gender: Gender.male,
      grad: Graduation.gcm,
    },
    {
      id: 452,
      gender: Gender.fem,
      grad: Graduation.gcm,
    },
    {
      id: 252,
      gender: Gender.male,
      grad: Graduation.gcm,
    },
  ],
});
console.timeEnd(lbl);

console.assert(out.assignState instanceof Array);
if (out.assignState.length > 0) {
  console.assert(typeof out.assignState[0].dayIndex === "number");
  console.assert(typeof out.assignState[0].dutyIndex === "number");
  console.assert(typeof out.assignState[0].workerId === "number");
}

console.log(out);

console.time("a");
