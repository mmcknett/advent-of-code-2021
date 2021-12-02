var readline = require('readline');
var rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
  terminal: false
});

let increaseCount = 0;
let sumIncreasedCount = 0;
const numbers = [];

rl.on('line', function(line) {
  const number = Number(line);
  numbers.push(number);
})

// Handles part 1 of the prompt
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

// Handles part 2 of the prompt
function countTriplesIncreases() {
  let previousSum = null;

  for (let i = 0; i < numbers.length - 2; ++i) {
    let incDecString = "(N/A - no previous sum)";
    const tripleSum = numbers[i] + numbers[i+1] + numbers[i+2];

    if (previousSum !== null) {
      let diff = tripleSum - previousSum;
      sumIncreasedCount += diff > 0 ? 1 : 0;

      incDecString = diff > 0 ? "(increased)" : diff < 0 ? "(decreased)" : "(no change)";
    }

    previousSum = tripleSum;
    console.log(tripleSum, incDecString);
  }
}

rl.on('close', function() {
  countIncreases();
  countTriplesIncreases();
  console.log("Number of increases:", increaseCount);
  console.log("Number of triple-sum increases:", sumIncreasedCount);
})
