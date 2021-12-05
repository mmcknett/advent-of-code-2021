const { RSA_PSS_SALTLEN_MAX_SIGN } = require('constants');
const readline = require('readline');
const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
  terminal: false
});

const horiz_and_vert_lines = [];
let max_x = 0, max_y = 0;

rl.on('line', function(line_read) {
  const line_strs = line_read.split("->").map(s => s.trim().split(","));
  const line = line_strs.map(([x, y]) => [Number(x), Number(y)]);
  const [[x1, y1], [x2, y2]] = line;
  horiz_and_vert_lines.push(line);

  max_x = Math.max(Math.max(x1, x2), max_x);
  max_y = Math.max(Math.max(y1, y2), max_y);
});

rl.on('close', function() {
  // console.log("lines:", horiz_and_vert_lines);
  console.log("lines count:", horiz_and_vert_lines.length);
  const heightmap = render();
  print(heightmap);
  console.log('maxes:', [max_x, max_y]);
  console.log("There are", overlapCount(heightmap), "overlaps.");
});

function render() {
  const heightmap = Array.from(
    { length: max_y + 1 },
    () => Array(max_x + 1).fill(0)
  );

  horiz_and_vert_lines.forEach(line => {
    drawLine(line, heightmap);
  })

  return heightmap;
}

function print(heightmap) {
  heightmap.forEach(row => {
    console.log(row.reduce(
      (s, val) => s + (val === 0 ? '.' : val), '')
    );
  });
}

function drawLine(line, heightmap) {
  let [[x1, y1], [x2, y2]] = line;
  if (x1 === x2) {
    // Vertical line
    if (y1 > y2) [y1, y2] = [y2, y1];
    for (let y = y1; y <= y2; ++y) {
      heightmap[y][x1] += 1;
    }
  } else if (y1 === y2) {
    // Horizontal line
    if (x1 > x2) [x1, x2] = [x2, x1];
    for (let x = x1; x <= x2; ++x) {
      heightmap[y1][x] += 1;
    }
  } else {
    // Diagonal
    const xincr = x1 < x2 ? 1 : -1;
    const yincr = y1 < y2 ? 1 : -1;
    for (let x = x1, y = y1; x != x2 + xincr, y != y2 + yincr; x += xincr, y += yincr) {
      heightmap[y][x] += 1;
    }
  }
}

function overlapCount(heightmap) {
  let overlaps = 0;
  heightmap.forEach(row => {
    row.forEach(val => {
      if (val > 1) {
        overlaps += 1;
      }
    });
  });
  return overlaps;
}
