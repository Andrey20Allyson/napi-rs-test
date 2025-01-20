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
  qualifier: {
    triesLimit: 14_000,
  },
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
    {
      id: 120,
      gender: Gender.male,
      grad: Graduation.sub,
    },
    {
      id: 654,
      gender: Gender.male,
      grad: Graduation.sub,
    },
    {
      id: 352,
      gender: Gender.male,
      grad: Graduation.insp,
    },
    {
      id: 359,
      gender: Gender.male,
      grad: Graduation.insp,
    },
  ],
});
console.timeEnd(lbl);

console.log("assigns: ", out.assignState.length);
