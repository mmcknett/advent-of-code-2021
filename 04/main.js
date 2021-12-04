const { exit } = require('process');
const readline = require('readline');
const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
  terminal: false
});

let numsInput = null;
let boards = [];

class Board {
  constructor() {
    this.squares = [];
    this.marks = [
      [false, false, false, false, false],
      [false, false, false, false, false],
      [false, false, false, false, false],
      [false, false, false, false, false],
      [false, false, false, false, false]
    ]
  }

  addRow(line) {
    this.squares.push(line.split(" ").filter(s => s !== "").map(s => Number(s.trim())));
  }

  mark(number) {
    for (let row = 0; row < this.marks.length; ++row) {
      for (let col = 0; col < this.marks.length; ++col) {
        if (this.squares[row][col] === number) {
          this.marks[row][col] = true;
        }
      }
    }
  }

  isWinner() {
    let wins = [];
    wins = wins.concat([...Array(5).keys()].map(col => this.isColumnWin(col)));
    wins = wins.concat([...Array(5).keys()].map(row => this.isRowWin(row)));
    return wins.some(w => w);
  }

  isColumnWin(col) {
    let marks = [];
    for (let row = 0; row < this.marks.length; ++row) {
      marks.push(this.marks[row][col]);
    }
    return marks.every(mark => mark);
  }

  isRowWin(row) {
    let marks = [];
    for (let col = 0; col < this.marks[row].length; ++col) {
      marks.push(this.marks[row][col]);
    }
    return marks.every(mark => mark);
  }

  score() {
    let sum = 0;
    for (let row = 0; row < this.marks.length; ++row) {
      for (let col = 0; col < this.marks.length; ++col) {
        if (!this.marks[row][col]) {
          sum += this.squares[row][col];
        }
      }
    }
    return sum;
  }

  toString() {
    let str = "";
    this.squares.forEach(row => {
      str += row.toString() + '\n';
    });
    str += '\n';
    return str;
  }

  print() {
    this.squares.forEach(row => {
      console.log(row);
    });
    console.log("");
  }
}

let boardRow = 0;

rl.on('line', function(line) {
  if (line.trim() === '') return;

  if (!numsInput) {
    numsInput = line.split(",").map(s => Number(s));
    console.log("nums input is", numsInput);
    return;
  }

  if (boardRow == 0) {
    console.log("pushing a board");
    boards.push(new Board())
  }

  boards[boards.length - 1].addRow(line);
  boardRow = ++boardRow % 5;
});

rl.on('close', function() {
  // console.log('boards are...\n', boards.toString());
  boards.forEach(board => board.print());

  for (const number of numsInput) {
    console.log("Checking", number);
    boards.forEach(board => {
      board.mark(number);
      if (board.isWinner()) {
        const boardscore = board.score();
        console.log("The winning board's unmarked score is:", boardscore);
        console.log("The final score is:", boardscore * number);
        exit();
      }
    });
  }
});