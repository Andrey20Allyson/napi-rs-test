const { Bench } = require('tinybench')

const { fibb } = require('../index.js')

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

const b = new Bench()

b.add('Native times 2', () => {
  fibb(100_000)
})

b.add('JavaScript times 2', () => {
  fibbJs(100_000)
})

b.run().then(() => console.table(b.table()))
