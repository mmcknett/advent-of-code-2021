const readline = require('readline');
const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
  terminal: false
});

let x = 0;
let depth = 0;

let correctedDepth = 0;
let aim = 0;

rl.on('line', function(line) {
  const [command, value] = line.split(" ");
  const num = Number(value);
  switch(command) {
    case 'forward': {
      x += num;
      correctedDepth += aim * num
      correctedDepth = Math.max(correctedDepth, 0);
      break;
    }
    case 'down': {
      depth += num;
      aim += num;
      break;
    }
    case 'up': {
      depth -= Math.min(depth, num);
      aim -= num;
      break;
    }
    default: throw Error('unknown command');
  }
});

rl.on('close', function() {
  console.log(
    'x:', x,
    ', depth:', depth,
    ', correctdDepth:', correctedDepth,
    ', multiplied:', x * depth,
    ', multiplied correctly:', x * correctedDepth);
});

