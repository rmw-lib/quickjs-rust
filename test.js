const try_run = (func, ...args) => {
  try {
    func(...args)
  } catch (err) {
    console.log('❌', err.message)
    console.log(err.stack)
  }
};

import * as rust from 'rust'
console.log("export from rust :", Object.keys(rust))

import {
  fib,
  sleep
} from 'rust'

(async () => {

  console.log('begin sleep 2s')
  await sleep(2000);
  console.log('sleep done')

  console.log('fib(3) =', fib(3));

  console.log("try catch example :")
  try_run(fib);
  try_run(fib, '*');

})()
