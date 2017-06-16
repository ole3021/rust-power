const readline = require('readline')

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
})

rl.question('Input the length of fibonacci you want to get? ', length => {
  rl.close()
  const startAt = new Date().getTime()
  const result = generateFib(length)
  const endAt = new Date().getTime()
  console.log(`>>> fibonacci generated in ${endAt - startAt} million seconds: ${result}`)
})


const generateFib = length => {
  let fibonacci = []

  for (let i = 0; i < length; i++) {
    fibonacci[i] = fib(i)
  }

  return fibonacci
}

const fib = index => {
  if (index <= 1) return index

  return fib(index - 2) + fib(index - 1)
}

// 一不小心好像写了个最优解。。。基本0ms出结果
// const generateFib = length => {
//   let fibonacci = [0, 1]
//   if (!Number.isInteger(Number(length))) throw new Error('Length is not a valid number')
//
//   if (length <= 2) return fibonacci.slice(0, length)
//
//   for (let i = 2; i < length; i++) {
//     fibonacci.push(fibonacci[i - 2] + fibonacci[i - 1])
//   }
//
//   return fibonacci
// }
