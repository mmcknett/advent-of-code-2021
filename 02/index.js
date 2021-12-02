const readline = require('readline');
const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
  terminal: false
});

let x = 0;
let depth = 0;

rl.on('line', function(line) {
  const [command, value] = line.split(" ");
  const num = Number(value);
  switch(command) {
    case 'forward': x += num; break;
    case 'down': depth += num; break;
    case 'up': depth -= Math.min(depth, num); break;
    default: throw Error('unknown command');
  }
});

rl.on('close', function() {
  console.log('x:', x, ', depth:', depth, ', multiplied:', x * depth);
});

