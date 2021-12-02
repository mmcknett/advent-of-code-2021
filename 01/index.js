var readline = require('readline');
var rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
  terminal: false
});

let increaseCount = 0;
let previousValue = null;

rl.on('line', function(line) {
  const number = Number(line);
  let incDecString = "(N/A - no previous measurement)";
  if (previousValue !== null) {
    let increased = (number > previousValue)
    increaseCount += increased ? 1 : 0;
    incDecString = increased ? "(increased)" : "(decreased)"
  }
  previousValue = number;
  console.log(line, incDecString);
})

rl.on('close', function() {
  console.log("Number of increases:", increaseCount);
})
