const lib = require("./dist/native/my-lib");

function dayOfWeek(year, month, day) {
  month += 1;
  day += 1;

  let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
  year -= month < 3;
  return (
    (year +
      Math.trunc(year / 4) -
      Math.trunc(year / 100) +
      Math.trunc(year / 400) +
      t.at(month - 1) +
      day) %
    7
  );
}

console.assert(lib.dayOfWeek(2031, 2, 0) === dayOfWeek(2031, 2, 0));
console.assert(lib.dayOfWeek(2025, 0, 4) === dayOfWeek(2025, 0, 4));
console.assert(lib.dayOfWeek(2025, 0, 0) === dayOfWeek(2025, 0, 0));
