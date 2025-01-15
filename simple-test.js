const { highestPrime } = require("./dist/native/my-lib");

function isPrime(number) {
  if (number <= 1) {
    return false;
  }

  const halfNumber = number / 2;
  for (let i = 2; i < halfNumber; i++) {
    if (number % i == 0) {
      return false;
    }
  }

  return true;
}

function highestPrimeJs(upper_limit) {
  let largest_prime = -1;

  for (let i = 2; i < upper_limit; i++) {
    if (isPrime(i)) {
      largest_prime = i;
    }
  }

  return largest_prime;
}

const input = 20_000;
let result;

console.time("Native Find Prime");
result = highestPrime(input);
console.timeEnd("Native Find Prime");
console.log(result);

console.time("JS Find Prime");
result = highestPrimeJs(input);
console.timeEnd("JS Find Prime");
console.log(result);
