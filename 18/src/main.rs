use std::ops;
use std::fmt;

fn main() {
    let x = fnum(1, 1) + fnum(2, 2) + fnum(3, 3) + fnum(4, 4);
    println!("{}", x);

    let x = fnum(1, 1) + fnum(2, 2) + fnum(3, 3) + fnum(4, 4) + fnum(5, 5);
    println!("{}", x);
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
        return L(0);
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
        return 0;
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
