const { times2, Car, walkWithCar } = require('./index')

// const arr1 = [1, 2, 3]
// const arr2 = times2(arr1)

// console.log(arr1)
// console.log(arr2)

const car = walkWithCar()

car.setName('Gol')

console.log(car.getName())
