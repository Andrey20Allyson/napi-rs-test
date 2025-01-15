const { Bench } = require('tinybench')

const { times2Num } = require('../index.js')

function times2Js(num) {
  return num * 2
}

const b = new Bench()

b.add('Native times 2', () => {
  times2Num(255)
})

b.add('JavaScript times 2', () => {
  times2Js(255)
})

b.run().then(() => console.table(b.table()))
