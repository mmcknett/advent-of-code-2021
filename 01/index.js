var readline = require('readline');
var rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
  terminal: false
});

let increaseCount = 0;
const numbers = [];

rl.on('line', function(line) {
  const number = Number(line);
  numbers.push(number);
})

function countIncreases() {
  let previousValue = null;

  for (number of numbers) {
    let incDecString = "(N/A - no previous measurement)";
    if (previousValue !== null) {
      let increased = (number > previousValue)
      increaseCount += increased ? 1 : 0;
      incDecString = increased ? "(increased)" : "(decreased)"
    }
    previousValue = number;
    // console.log(number, incDecString);
  }
}

function countTriplesIncreases() {
  
}

rl.on('close', function() {
  countIncreases();
  console.log("Number of increases:", increaseCount);
})
