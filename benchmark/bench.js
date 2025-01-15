const { Bench } = require('tinybench')

const { fibb, highestPrime } = require('../native-dist/my-lib')

function fibbJs(pos) {
  let current = 0
  let prev = 1

  for (let i = 0; i < pos; i++) {
    let temp = current
    current = current + prev
    prev = temp
  }

  return current
}

function isPrime(number) {
  if (number <= 1) {
    return false
  }

  const halfNumber = number / 2
  for (let i = 2; i < halfNumber; i++) {
    if (number % i == 0) {
      return false
    }
  }

  return true
}

function highestPrimeJs(upper_limit) {
  let largest_prime = -1

  for (let i = 2; i < upper_limit; i++) {
    if (isPrime(i)) {
      largest_prime = i
    }
  }

  return largest_prime
}

const b = new Bench()

b.add('Native times 2', () => {
  highestPrime(3000)
})

b.add('JavaScript times 2', () => {
  highestPrimeJs(3000)
})

b.run().then(() => console.table(b.table()))
