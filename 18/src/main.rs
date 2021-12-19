use std::ops;
use std::cmp;
use std::fmt;

fn main() {
    let x = fnum(1, 1) + fnum(2, 2) + fnum(3, 3) + fnum(4, 4);
    println!("{}", x);

    let x = fnum(1, 1) + fnum(2, 2) + fnum(3, 3) + fnum(4, 4) + fnum(5, 5);
    println!("{}", x);

    let x = fnum(1, 1) + fnum(2, 2) + fnum(3, 3) + fnum(4, 4) + fnum(5, 5) + fnum(6, 6);
    println!("{}", x);

    let start = Fnum::parse("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]");
    let nums = vec![
        "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
        "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
        "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
        "[7,[5,[[3,8],[1,4]]]]",
        "[[2,[2,2]],[8,[8,1]]]",
        "[2,9]",
        "[1,[[[9,3],9],[[9,0],[0,7]]]]",
        "[[[5,[7,4]],7],1]",
        "[[[[4,2],2],6],[8,7]]",
    ];
    let sum = nums.iter().map(|s| Fnum::parse(s)).fold(start, |s, a| s + a);
    println!("{}", sum);

    let num = Fnum::parse("[[1,2],[[3,4],5]]");
    println!("{} magnitude: {}", num, num.magnitude());

    part1();
    part2();
}

fn part1() {
    let start = Fnum::parse("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]");
    let nums = vec![
        "[[[5,[2,8]],4],[5,[[9,9],0]]]",
        "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
        "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
        "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
        "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
        "[[[[5,4],[7,7]],8],[[8,3],8]]",
        "[[9,3],[[9,9],[6,[4,9]]]]",
        "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
        "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"
    ];
    let sum = nums.iter().map(|s| Fnum::parse(s)).fold(start, |s, a| s + a);
    println!("Example: {}, magnitude: {}", sum, sum.magnitude());

    let start = Fnum::parse("[[6,[[9,4],[5,5]]],[[[0,7],[7,8]],[7,0]]]");
    let nums = vec![
        "[[[[2,1],[8,6]],[2,[4,0]]],[9,[4,[0,6]]]]",
        "[[[[4,2],[7,7]],4],[3,5]]",
        "[8,[3,[[2,3],5]]]",
        "[[[[0,0],[4,7]],[[5,5],[8,5]]],[8,0]]",
        "[[[[5,2],[5,7]],[1,[5,3]]],[[4,[8,4]],2]]",
        "[[5,[[2,8],[9,3]]],[[7,[5,2]],[[9,0],[5,2]]]]",
        "[[9,[[4,3],1]],[[[9,0],[5,8]],[[2,6],1]]]",
        "[[0,6],[6,[[6,4],[7,0]]]]",
        "[[[9,[4,2]],[[6,0],[8,9]]],[[0,4],[3,[6,8]]]]",
        "[[[[3,2],0],[[9,6],[3,1]]],[[[3,6],[7,6]],[2,[6,4]]]]",
        "[5,[[[1,6],[7,8]],[[6,1],[3,0]]]]",
        "[2,[[6,[7,6]],[[8,6],3]]]",
        "[[[[0,9],1],[2,3]],[[[7,9],1],7]]",
        "[[[[1,8],3],[[8,8],[0,8]]],[[2,1],[8,0]]]",
        "[[2,9],[[5,1],[[9,3],[4,0]]]]",
        "[9,[8,4]]",
        "[[[3,3],[[6,2],8]],5]",
        "[[[9,[4,8]],[[1,3],[6,7]]],[9,[[4,4],2]]]",
        "[[[[1,3],6],[[5,6],[1,9]]],[9,[[0,2],9]]]",
        "[7,[[[0,6],[1,2]],4]]",
        "[[[[5,0],[8,7]],[[7,3],0]],[[6,7],[0,1]]]",
        "[[[[5,4],7],[[8,2],1]],[[[7,0],[6,9]],0]]",
        "[[[3,[5,6]],[[9,5],4]],[[[9,4],[8,1]],[5,[7,4]]]]",
        "[[[3,[7,5]],[[8,1],8]],[[[6,3],[9,2]],[[5,7],7]]]",
        "[8,[[2,0],[[2,6],8]]]",
        "[[[[5,8],9],1],[9,6]]",
        "[[[9,9],[8,8]],[[[3,5],[8,0]],[[4,6],[3,2]]]]",
        "[[5,[[5,1],6]],[[5,8],9]]",
        "[[7,[[1,6],6]],[[[8,6],7],[6,6]]]",
        "[[0,[[9,5],0]],[4,[[7,9],[4,9]]]]",
        "[[[[4,3],[3,5]],[[1,9],[7,6]]],[3,[[6,4],[6,0]]]]",
        "[[[2,6],6],[6,3]]",
        "[[[[1,5],[3,7]],0],[3,7]]",
        "[4,[[[5,5],4],[[5,5],[9,3]]]]",
        "[[3,[8,6]],[8,[7,7]]]",
        "[8,[9,5]]",
        "[[[6,3],[2,[3,6]]],[[[6,0],[0,2]],[[8,7],5]]]",
        "[[[8,[1,2]],2],7]",
        "[[[[8,4],[2,7]],[[3,9],7]],[[4,[8,8]],[[7,4],9]]]",
        "[[[8,[2,5]],[3,[1,2]]],[[4,[5,0]],3]]",
        "[[8,[0,3]],[[5,1],[1,1]]]",
        "[[[8,[3,6]],6],[[7,[1,5]],[[4,8],9]]]",
        "[[[5,0],[0,3]],[[2,[7,8]],[1,[4,8]]]]",
        "[9,[4,[9,4]]]",
        "[[[9,[0,4]],2],3]",
        "[[9,[7,[8,9]]],3]",
        "[[[8,6],[[3,5],[9,2]]],[[3,[9,7]],5]]",
        "[[6,[[7,4],2]],[2,[7,[6,0]]]]",
        "[1,[[[2,2],6],8]]",
        "[[[6,[1,8]],[[9,3],[1,8]]],[[[8,2],[9,3]],[[8,2],[9,9]]]]",
        "[[[[2,9],[1,7]],[[4,0],8]],[[8,9],[6,3]]]",
        "[[[[2,4],[6,1]],[[5,4],[2,8]]],[8,[1,[2,4]]]]",
        "[[[4,6],[1,6]],[3,[1,1]]]",
        "[[[[8,3],8],8],[1,[[4,2],3]]]",
        "[[[9,[8,7]],[5,9]],[8,[[5,6],[4,5]]]]",
        "[[[[4,1],2],[[7,8],4]],[0,6]]",
        "[[[9,7],[[8,6],[6,9]]],[[8,[8,4]],[[9,0],2]]]",
        "[[[8,5],[1,9]],[[[2,4],5],6]]",
        "[[[9,[9,3]],[9,[2,3]]],[7,7]]",
        "[[[8,[7,4]],[2,6]],[[[4,5],[9,9]],[0,[5,2]]]]",
        "[7,[2,2]]",
        "[[[[1,8],[5,2]],3],[0,[2,[4,5]]]]",
        "[[5,[[4,8],[5,5]]],[4,[[3,4],[6,0]]]]",
        "[[3,1],[4,[3,[8,2]]]]",
        "[[3,7],[3,[[6,1],[0,2]]]]",
        "[[4,[6,2]],[[3,9],8]]",
        "[[[[2,9],3],[[5,6],4]],[8,2]]",
        "[[4,[[7,9],[4,9]]],[[4,3],[7,[0,7]]]]",
        "[[[3,[8,9]],[[3,4],[9,5]]],3]",
        "[0,[[[3,0],[8,7]],[[0,9],[9,1]]]]",
        "[[[5,[9,9]],2],[4,8]]",
        "[[[[4,4],4],5],[3,4]]",
        "[[[3,[2,2]],7],[[3,2],0]]",
        "[[[[0,5],[5,2]],2],[2,[[1,2],2]]]",
        "[[[4,6],6],[[0,1],6]]",
        "[2,[[[3,9],7],[[9,8],8]]]",
        "[[7,9],[7,[[3,0],9]]]",
        "[[[1,[6,2]],[0,8]],[[[7,2],4],9]]",
        "[[[[4,7],[1,5]],[5,9]],[[2,[0,4]],[7,[7,0]]]]",
        "[[1,[[2,0],[0,4]]],[[[4,6],9],[[6,8],[0,1]]]]",
        "[[[[6,0],7],[7,[9,6]]],[[7,[4,9]],[9,4]]]",
        "[[[5,[4,6]],[[1,9],[5,8]]],[[[3,6],[2,6]],[[7,3],7]]]",
        "[[[6,0],[6,6]],[2,8]]",
        "[[[4,[7,2]],[[5,6],[2,4]]],[[[6,8],5],[4,6]]]",
        "[[[[9,0],9],[4,0]],[[[9,1],8],[6,4]]]",
        "[[6,3],[1,[[5,0],[9,9]]]]",
        "[[[2,7],[5,6]],[[6,[1,4]],[9,9]]]",
        "[[[[0,5],3],[8,7]],[[[9,9],[6,2]],[0,7]]]",
        "[[[5,6],[1,7]],[[[0,4],9],9]]",
        "[[[7,3],3],[6,[0,[8,9]]]]",
        "[[[0,6],[[8,5],[4,6]]],[[[2,7],[4,2]],[[8,7],[0,5]]]]",
        "[[[8,[7,3]],1],8]",
        "[[8,[8,[8,2]]],[[5,4],[1,[2,6]]]]",
        "[[[[1,1],[8,6]],5],9]",
        "[[[[2,4],[5,7]],[[5,8],[3,1]]],7]",
        "[[4,[[0,1],9]],[[3,8],[4,2]]]",
        "[3,2]",
        "[[3,4],[8,[[6,5],[6,6]]]]",
        "[[[[7,0],[3,8]],[[3,3],[2,6]]],[[8,0],9]]"
    ];
    let sum = nums.iter().map(|s| Fnum::parse(s)).fold(start, |s, a| s + a);
    println!("Example: {}, magnitude: {}", sum, sum.magnitude());
}

fn part2() {
    // Example
    let mut max_pair_sum = 0;

    // let nums_example = vec![
    //     "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
    //     "[[[5,[2,8]],4],[5,[[9,9],0]]]",
    //     "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
    //     "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
    //     "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
    //     "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
    //     "[[[[5,4],[7,7]],8],[[8,3],8]]",
    //     "[[9,3],[[9,9],[6,[4,9]]]]",
    //     "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
    //     "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"
    // ];

    let nums_input = vec![
        "[[6,[[9,4],[5,5]]],[[[0,7],[7,8]],[7,0]]]",
        "[[[[2,1],[8,6]],[2,[4,0]]],[9,[4,[0,6]]]]",
        "[[[[4,2],[7,7]],4],[3,5]]",
        "[8,[3,[[2,3],5]]]",
        "[[[[0,0],[4,7]],[[5,5],[8,5]]],[8,0]]",
        "[[[[5,2],[5,7]],[1,[5,3]]],[[4,[8,4]],2]]",
        "[[5,[[2,8],[9,3]]],[[7,[5,2]],[[9,0],[5,2]]]]",
        "[[9,[[4,3],1]],[[[9,0],[5,8]],[[2,6],1]]]",
        "[[0,6],[6,[[6,4],[7,0]]]]",
        "[[[9,[4,2]],[[6,0],[8,9]]],[[0,4],[3,[6,8]]]]",
        "[[[[3,2],0],[[9,6],[3,1]]],[[[3,6],[7,6]],[2,[6,4]]]]",
        "[5,[[[1,6],[7,8]],[[6,1],[3,0]]]]",
        "[2,[[6,[7,6]],[[8,6],3]]]",
        "[[[[0,9],1],[2,3]],[[[7,9],1],7]]",
        "[[[[1,8],3],[[8,8],[0,8]]],[[2,1],[8,0]]]",
        "[[2,9],[[5,1],[[9,3],[4,0]]]]",
        "[9,[8,4]]",
        "[[[3,3],[[6,2],8]],5]",
        "[[[9,[4,8]],[[1,3],[6,7]]],[9,[[4,4],2]]]",
        "[[[[1,3],6],[[5,6],[1,9]]],[9,[[0,2],9]]]",
        "[7,[[[0,6],[1,2]],4]]",
        "[[[[5,0],[8,7]],[[7,3],0]],[[6,7],[0,1]]]",
        "[[[[5,4],7],[[8,2],1]],[[[7,0],[6,9]],0]]",
        "[[[3,[5,6]],[[9,5],4]],[[[9,4],[8,1]],[5,[7,4]]]]",
        "[[[3,[7,5]],[[8,1],8]],[[[6,3],[9,2]],[[5,7],7]]]",
        "[8,[[2,0],[[2,6],8]]]",
        "[[[[5,8],9],1],[9,6]]",
        "[[[9,9],[8,8]],[[[3,5],[8,0]],[[4,6],[3,2]]]]",
        "[[5,[[5,1],6]],[[5,8],9]]",
        "[[7,[[1,6],6]],[[[8,6],7],[6,6]]]",
        "[[0,[[9,5],0]],[4,[[7,9],[4,9]]]]",
        "[[[[4,3],[3,5]],[[1,9],[7,6]]],[3,[[6,4],[6,0]]]]",
        "[[[2,6],6],[6,3]]",
        "[[[[1,5],[3,7]],0],[3,7]]",
        "[4,[[[5,5],4],[[5,5],[9,3]]]]",
        "[[3,[8,6]],[8,[7,7]]]",
        "[8,[9,5]]",
        "[[[6,3],[2,[3,6]]],[[[6,0],[0,2]],[[8,7],5]]]",
        "[[[8,[1,2]],2],7]",
        "[[[[8,4],[2,7]],[[3,9],7]],[[4,[8,8]],[[7,4],9]]]",
        "[[[8,[2,5]],[3,[1,2]]],[[4,[5,0]],3]]",
        "[[8,[0,3]],[[5,1],[1,1]]]",
        "[[[8,[3,6]],6],[[7,[1,5]],[[4,8],9]]]",
        "[[[5,0],[0,3]],[[2,[7,8]],[1,[4,8]]]]",
        "[9,[4,[9,4]]]",
        "[[[9,[0,4]],2],3]",
        "[[9,[7,[8,9]]],3]",
        "[[[8,6],[[3,5],[9,2]]],[[3,[9,7]],5]]",
        "[[6,[[7,4],2]],[2,[7,[6,0]]]]",
        "[1,[[[2,2],6],8]]",
        "[[[6,[1,8]],[[9,3],[1,8]]],[[[8,2],[9,3]],[[8,2],[9,9]]]]",
        "[[[[2,9],[1,7]],[[4,0],8]],[[8,9],[6,3]]]",
        "[[[[2,4],[6,1]],[[5,4],[2,8]]],[8,[1,[2,4]]]]",
        "[[[4,6],[1,6]],[3,[1,1]]]",
        "[[[[8,3],8],8],[1,[[4,2],3]]]",
        "[[[9,[8,7]],[5,9]],[8,[[5,6],[4,5]]]]",
        "[[[[4,1],2],[[7,8],4]],[0,6]]",
        "[[[9,7],[[8,6],[6,9]]],[[8,[8,4]],[[9,0],2]]]",
        "[[[8,5],[1,9]],[[[2,4],5],6]]",
        "[[[9,[9,3]],[9,[2,3]]],[7,7]]",
        "[[[8,[7,4]],[2,6]],[[[4,5],[9,9]],[0,[5,2]]]]",
        "[7,[2,2]]",
        "[[[[1,8],[5,2]],3],[0,[2,[4,5]]]]",
        "[[5,[[4,8],[5,5]]],[4,[[3,4],[6,0]]]]",
        "[[3,1],[4,[3,[8,2]]]]",
        "[[3,7],[3,[[6,1],[0,2]]]]",
        "[[4,[6,2]],[[3,9],8]]",
        "[[[[2,9],3],[[5,6],4]],[8,2]]",
        "[[4,[[7,9],[4,9]]],[[4,3],[7,[0,7]]]]",
        "[[[3,[8,9]],[[3,4],[9,5]]],3]",
        "[0,[[[3,0],[8,7]],[[0,9],[9,1]]]]",
        "[[[5,[9,9]],2],[4,8]]",
        "[[[[4,4],4],5],[3,4]]",
        "[[[3,[2,2]],7],[[3,2],0]]",
        "[[[[0,5],[5,2]],2],[2,[[1,2],2]]]",
        "[[[4,6],6],[[0,1],6]]",
        "[2,[[[3,9],7],[[9,8],8]]]",
        "[[7,9],[7,[[3,0],9]]]",
        "[[[1,[6,2]],[0,8]],[[[7,2],4],9]]",
        "[[[[4,7],[1,5]],[5,9]],[[2,[0,4]],[7,[7,0]]]]",
        "[[1,[[2,0],[0,4]]],[[[4,6],9],[[6,8],[0,1]]]]",
        "[[[[6,0],7],[7,[9,6]]],[[7,[4,9]],[9,4]]]",
        "[[[5,[4,6]],[[1,9],[5,8]]],[[[3,6],[2,6]],[[7,3],7]]]",
        "[[[6,0],[6,6]],[2,8]]",
        "[[[4,[7,2]],[[5,6],[2,4]]],[[[6,8],5],[4,6]]]",
        "[[[[9,0],9],[4,0]],[[[9,1],8],[6,4]]]",
        "[[6,3],[1,[[5,0],[9,9]]]]",
        "[[[2,7],[5,6]],[[6,[1,4]],[9,9]]]",
        "[[[[0,5],3],[8,7]],[[[9,9],[6,2]],[0,7]]]",
        "[[[5,6],[1,7]],[[[0,4],9],9]]",
        "[[[7,3],3],[6,[0,[8,9]]]]",
        "[[[0,6],[[8,5],[4,6]]],[[[2,7],[4,2]],[[8,7],[0,5]]]]",
        "[[[8,[7,3]],1],8]",
        "[[8,[8,[8,2]]],[[5,4],[1,[2,6]]]]",
        "[[[[1,1],[8,6]],5],9]",
        "[[[[2,4],[5,7]],[[5,8],[3,1]]],7]",
        "[[4,[[0,1],9]],[[3,8],[4,2]]]",
        "[3,2]",
        "[[3,4],[8,[[6,5],[6,6]]]]",
        "[[[[7,0],[3,8]],[[3,3],[2,6]]],[[8,0],9]]"
    ];

    let nums = nums_input;

    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if i == j { continue; }
            let candidate = Fnum::parse(nums[i]) + Fnum::parse(nums[j]);
            max_pair_sum = cmp::max(max_pair_sum, candidate.magnitude());
        }
    }

    println!("Part 2");
    println!("Max magnitude is {}", max_pair_sum);
}

// enum Fnum {
//     Lit(u64),
//     Pair(Box<Fnum>, Box<Fnum>)
// }
// type Fnum = (u64, u64);

#[derive(Debug, PartialEq)]
enum Fnum{
    L(u64),
    P(Box<Fnum>, Box<Fnum>)
}

impl fmt::Display for Fnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            L(val) => write!(f, "{}", val),
            P(left, right) => write!(f, "[{}, {}]", left, right)
        }
    }
}

use Fnum::{L, P};

fn fnum(l: u64, r: u64) -> Fnum{
    P(
        Box::new(L(l)),
        Box::new(L(r))
    )
}

impl Fnum {
    fn parse(input: &str) -> Self {
        let mut iter = input.chars();
        iter.next(); // Assume the first character is '['
        return Fnum::parse_pair(&mut iter);
    }

    fn parse_pair<'a>(iter: &mut std::str::Chars<'a>) -> Self {
        let left = match iter.next().unwrap() {
            '[' => Box::new(Fnum::parse_pair(iter)),
            val => Box::new(L(val.to_digit(10).unwrap() as u64)),
            _ => panic!("Invalid pair")
        };
        iter.next(); // eat ','
        let right = match iter.next().unwrap() {
            '[' => Box::new(Fnum::parse_pair(iter)),
            val => Box::new(L(val.to_digit(10).unwrap() as u64)),
            _ => panic!("Invalid pair")
        };
        iter.next(); // eat ']'

        P(left, right)
    }

    fn is_literal(&self) -> bool {
        match self {
            L(_) => true,
            P(_, _) => false
        }
    }

    fn get_pair(&self) -> Option<(u64, u64)> {
        match self {
            P(left, right) => 
            {
                if let L(ll) = **left {
                    if let L(rr) = **right {
                        return Some((ll, rr));
                    }
                }
                None
            },
            _ => None
        }
    }

    fn split(&mut self) {
        match self {
            L(val) => {
                let avg = *val / 2;
                *self = P(
                    Box::new(L(avg)),
                    Box::new(L(*val - avg))
                )
            },
            P(_, _) => ()
        }
    }

    // Returns true if a split occurs.
    fn split_rec(&mut self) -> bool {
        match self {
            L(val) => {
                if *val >= 10 {
                    self.split();
                    true
                } else {
                    false
                }
            },
            P(left, right) => {
                left.split_rec() || right.split_rec()
            }
        }
    }

    // // [2, [3, [[[5, 2], 2], 4]], [3, 2]]
    // // Explode makes the number become Lit(0), and the number's
    // // left & right become residuals that either get applied to a value
    // // going back down the stack, or get push up the bottom's left/right side of the stack.
    // fn explode(&mut self) {
    //     let mut stack = vec![&self];
    //     let left_residual = 0;
    //     let right_residual = 0;

    //     // match self {
    //     //     Pair(Lit(_), Lit(_)) => return (),
    //     //     Pair(Lit(left), right) => right.explode_rec(1),
    //     //     Pair(left, Lit(right)) => left.explode_rec(1)
    //     // };
    // }

    fn add(&mut self, b: u64) {
        match self {
            L(val) => *val += b,
            P(_, _) => panic!("Can't add to a pair")
        }
    }

    fn push_residue_left(&mut self, residue: u64) {
        if residue == 0 {
            return;
        }

        match self {
            L(literal) => *literal += residue,
            P(left, _) => left.push_residue_left(residue)
        }
    }

    fn push_residue_right(&mut self, residue: u64) {
        if residue == 0 {
            return;
        }

        match self {
            L(literal) => *literal += residue,
            P(_, right) => right.push_residue_right(residue)
        }
    }

    fn explode_rec(&mut self, depth: u32) -> Option<(u64, u64)> {
        // Cases:
        //   - Both literals: if the depth is 5+, the pair becomes a Lit & residuals are returned
        //   - left literal, right pair: recursive call right, add residual to left, return (0, right_res)
        //   - left pair, right literal: recursive call left, add residual to right, return (left_res, 0)

        if let Some(pair) = self.get_pair() {
            if depth > 4 {
                *self = L(0);
                return Some(pair);
            } else {
                return None; // Should actually return None...to indicate that we should go up the other side, potentially.
            }
        }

        match self {
            P(left, right) => {
                if !left.is_literal() && !right.is_literal() {
                    // both are pairs
                    // 1. Try expoding left.
                    // 2. If it explodes, push the residue up right.
                    // 3. If not, try exploding right. Push any residue up right.
                    match left.explode_rec(depth + 1) {
                        Some((res_l, res_r)) => {
                            right.push_residue_left(res_r);
                            Some((res_l, 0))
                        },
                        None => {
                            match right.explode_rec(depth + 1) {
                                Some((res_l, res_r)) => {
                                    left.push_residue_right(res_l);
                                    Some((0, res_r))
                                },
                                None => None
                            }
                        }
                    }
                } else if left.is_literal() {
                    // right is a Pair.
                    let res = right.explode_rec(depth + 1);
                    match res {
                        Some((res_l, res_r)) => {
                            left.add(res_l);
                            Some((0, res_r))
                        },
                        None => None
                    }
                } else {
                    // left is a Pair.
                    let res = left.explode_rec(depth + 1);
                    match res {
                        Some((res_l, res_r)) => {
                            right.add(res_r);
                            Some((res_l, 0))
                        },
                        None => None
                    }
                }
            },
            L(_) => None
        }
    }

    fn reduce(&mut self) {
        // Explode, then split, until there are not explodes/splits left.
        while self.explode_rec(1).is_some() || self.split_rec() {}
    }

    fn magnitude(&self) -> u64 {
        match self {
            L(val) => *val,
            P(left, right) => 3 * left.magnitude() + 2 * right.magnitude()
        }
    }
}

impl ops::Add<Fnum> for Fnum {
    type Output = Fnum;

    fn add(self, _rhs: Fnum) -> Fnum {
        let mut sum = P(
            Box::new(self),
            Box::new(_rhs)
        );
        sum.reduce();
        return sum;
    }
}


#[cfg(test)]
mod tests {
    use super::{
        Fnum,
        fnum
    };

    use Fnum::{L, P};

    #[test]
    fn split() {
        let mut num = L(3);
        num.split();
        assert_eq!(fnum(1, 2), num);
    }

    #[test]
    fn explode_depth_5() {
        let mut num = P(
            Box::new(L(3)),
            Box::new(L(4))
        );

        let res = num.explode_rec(5);

        assert_eq!(L(0), num);
        assert_eq!(Some((3, 4)), res);
    }

    #[test]
    fn explode_depth_4() {
        let mut num = P(
            Box::new(L(3)),
            Box::new(L(4))
        );

        let res = num.explode_rec(4);

        assert_eq!(P(
            Box::new(L(3)),
            Box::new(L(4))
        ), num);
        assert_eq!(None, res);
    }

    #[test]
    fn explode_depth_4_pair_right() {
        let mut num = P(
            Box::new(L(3)),
            Box::new(P(
                Box::new(L(5)),
                Box::new(L(4))
            ))
        );

        let res = num.explode_rec(4);

        assert_eq!(P(
            Box::new(L(8)),
            Box::new(L(0))
        ), num);
        assert_eq!(Some((0, 4)), res);
    }

    #[test]
    fn explode_depth_4_pair_left() {
        let mut num = P(
            Box::new(P(
                Box::new(L(5)),
                Box::new(L(4))
            )),
            Box::new(L(3))
        );

        let res = num.explode_rec(4);

        assert_eq!(P(
            Box::new(L(0)),
            Box::new(L(7))
        ), num);
        assert_eq!(Some((5, 0)), res);
    }

    #[test]
    fn explode_depth_3_pairs() {
        let mut num = Box::new(P(
            Box::new(L(2)),
            Box::new(P(
                Box::new(P(
                    Box::new(L(7)),
                    Box::new(L(4))
                )),
                Box::new(L(3))
            ))
        ));

        let res = num.explode_rec(3);

        assert_eq!(Box::new(P(
            Box::new(L(9)),
            Box::new(P(
                Box::new(L(0)),
                Box::new(L(7))
            ))
        )), num);
        assert_eq!(Some((0, 0)), res);
    }

    #[test]
    fn explode_depth_2_pairs_no_explode() {
        let mut num = Box::new(P(
            Box::new(L(2)),
            Box::new(P(
                Box::new(P(
                    Box::new(L(7)),
                    Box::new(L(4))
                )),
                Box::new(L(3))
            ))
        ));

        let res = num.explode_rec(2);

        assert_eq!(Box::new(P(
            Box::new(L(2)),
            Box::new(P(
                Box::new(P(
                    Box::new(L(7)),
                    Box::new(L(4))
                )),
                Box::new(L(3))
            ))
        )), num);
        assert_eq!(None, res);
    }

    #[test]
    fn explode_upward_left() {
        let mut num =
            Box::new(P(
                Box::new(P(
                    Box::new(L(7)),
                    Box::new(L(4))
                )),
                Box::new(P(
                    Box::new(P(
                        Box::new(L(7)),
                        Box::new(L(4))
                    )),
                    Box::new(L(3))
                ))
            ));

        let res = num.explode_rec(3);

        assert_eq!(Box::new(P(
            Box::new(P(
                Box::new(L(7)),
                Box::new(L(11))
            )),
            Box::new(P(
                Box::new(L(0)),
                Box::new(L(7))
            ))
        )), num);
        assert_eq!(Some((0, 0)), res);
    }

    #[test]
    fn explode_upward_right() {
        let mut num =
            Box::new(P(
                Box::new(P(
                    Box::new(L(3)),
                    Box::new(P(
                        Box::new(L(7)),
                        Box::new(L(4))
                    ))
                )),
                Box::new(P(
                    Box::new(L(7)),
                    Box::new(L(4))
                ))
            ));

        let res = num.explode_rec(3);

        assert_eq!(Box::new(P(
            Box::new(P(
                Box::new(L(10)),
                Box::new(L(0))
            )),
            Box::new(P(
                Box::new(L(11)),
                Box::new(L(4))
            ))
        )), num);
        assert_eq!(Some((0, 0)), res);
    }

    #[test]
    fn get_pair() {
        let num = P(
            Box::new(L(3)),
            Box::new(L(4))
        );

        assert_eq!(Some((3, 4)), num.get_pair());
    }

    #[test]
    fn get_pair_2() {
        let num = P(
            Box::new(P(
                Box::new(L(3)),
                Box::new(L(4))
            )),
            Box::new(L(4))
        );

        assert_eq!(None, num.get_pair());
    }

    #[test]
    fn get_pair_3() {
        let num = L(3);

        assert_eq!(None, num.get_pair());
    }

    #[test]
    fn push_left() {
        let mut num = P(
            Box::new(P(
                Box::new(L(3)),
                Box::new(L(4))
            )),
            Box::new(L(4))
        );

        num.push_residue_left(8);

        assert_eq!(
            P(
                Box::new(P(
                    Box::new(L(11)),
                    Box::new(L(4))
                )),
                Box::new(L(4))
            ),
            num
        );
    }

    #[test]
    fn magnitudes() {
        assert_eq!(143,  Fnum::parse("[[1,2],[[3,4],5]]").magnitude());
        assert_eq!(1384, Fnum::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").magnitude());
        assert_eq!(445,  Fnum::parse("[[[[1,1],[2,2]],[3,3]],[4,4]]").magnitude());
        assert_eq!(791,  Fnum::parse("[[[[3,0],[5,3]],[4,4]],[5,5]]").magnitude());
        assert_eq!(1137, Fnum::parse("[[[[5,0],[7,4]],[5,5]],[6,6]]").magnitude());
        assert_eq!(3488, Fnum::parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").magnitude());
    }

    // #[test]
    // fn magnitude_1() {
    //     assert_eq!(143, magnitude(Fnum::parse("[[1,2],[[3,4],5]]")));
    // }

    // #[test]
    // fn mag_2() {
    //     assert_eq!(1384, magnitude(Fnum::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")));
    // }

    // #[test]
    // fn mag_3() {
    //     assert_eq!(445, magnitude(Fnum::parse("[[[[1,1],[2,2]],[3,3]],[4,4]]")));
    // }

    // #[test]
    // fn mag_4() {
    //     assert_eq!(791, magnitude(Fnum::parse("[[[[3,0],[5,3]],[4,4]],[5,5]]")));
    // }

    // #[test]
    // fn mag_5() {
    //     assert_eq!(1137, magnitude(Fnum::parse("[[[[5,0],[7,4]],[5,5]],[6,6]]")));
    // }

    // #[test]
    // fn mag_6() {
    //     assert_eq!(3488, magnitude(Fnum::parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")));
    // }
}
