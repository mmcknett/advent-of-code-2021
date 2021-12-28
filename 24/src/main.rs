use std::collections::HashMap;
use std::io::{self, BufRead};
use std::cmp::Ordering;

fn main() {
    let mut aluvm = Aluvm::new();
    aluvm.load_instructions();

    // Example 1
    // aluvm.run_on_input(vec![-90]);
    // println!("expected: 90, actual: {}", aluvm.vars.get("x").unwrap());

    // Example 2
    // aluvm.run_on_input(vec![3, 9]);
    // println!("expected: 1, actual: {}", aluvm.vars.get("z").unwrap());

    // aluvm.reset();
    // aluvm.run_on_input(vec![3, 8]);
    // println!("expected: 0, actual: {}", aluvm.vars.get("z").unwrap());

    // Example 3
    // aluvm.run_on_input(vec![0xB]);
    // println!("expected: 1011, actual: {}{}{}{}",
    //     aluvm.vars.get("w").unwrap(),
    //     aluvm.vars.get("x").unwrap(),
    //     aluvm.vars.get("y").unwrap(),
    //     aluvm.vars.get("z").unwrap(),
    // );


    // for i in 1..=9 {
    //     for j in 1..=9 {
    //         aluvm.run_on_input(vec![i, j]);
    //         aluvm.printvars();
    //         aluvm.reset();
    //     }
    // }

    // let example = 13579246899999i64;
    // aluvm.run_on_input(&get_as_vec(example));
    // println!("For the example {}, z is {}", example, aluvm.vars.get("z").unwrap());
    // aluvm.reset();

    // let hi_so_far = 13997997296975i64;
    // 13997991296953 is too low!

    // Part 1...
    // See transcriptions.txt -- This number was arrived at by looking at how
    // the x value, as observed during the x == w check, changed across each run.
    // Any time changing a digit made it so an x value came within 1..=9, the digit
    // was modified to make it so that the x value was its highest possible.
    // let hi_so_far = 939_9_7_99_929_69_1_2i64;

    // Part 2...
    // Doing some exploration turns up that you have to have at least 8 as
    // the first digit.
    // I came up with the necessary prefix 8111137, and brute-forced the remaining
    // digits from that point.
    // let hi_so_far = 811_1_1_37_194_11_1_1i64;
    let hi_so_far = 811_1_1_37_914_18_1_1i64;

    aluvm.run_on_input(&get_as_vec(hi_so_far));
    println!("");
    aluvm.printvars();
    println!("For the hi so far {}, z is {}, as base26: {}", hi_so_far, aluvm.vars.get("z").unwrap(), aluvm.get_z_base26());
    aluvm.reset();

    // part_1_by_parts(aluvm);

    // Used this to brute-force part 2.
    // run_part1(aluvm);
}

fn part_1_by_parts(mut aluvm: Aluvm) {
    let mut results = vec![];

    const TAKE: usize = 100000;

    for i in 1..=9 {
        // for i2 in 1..=9 {
        //     for i3 in 1..=9 {
        //         for i4 in 1..=9 {
        //             for i5 in 1..=9 {
        //                 for i6 in 1..=9 {
                            // for i7 in 1..=9 {
                                // for idx8 in 1..=9 {
                                    //for i9 in 1..=9 {
                                        let input = vec![i]; //, i2, i3, i4, i5, i6]; //, i7]; //, idx8]; //, i9];
                                        aluvm.reset();
                                        match aluvm.run_on_input(&input) {
                                            Ok(_) => (),
                                            Err(_) => ()
                                        };
                                    
                                        let z = *aluvm.vars.get("z").unwrap();
        
                                        results.push((z, input));
                                    //}
                                // }
                            // }
        //                 }
        //             }
        //         }
        //     }
        // }
    }

    let sortfn = |(a_z, a_vec): &(i64, Vec<i64>), (b_z, b_vec): &(i64, Vec<i64>)| {
        let zcmp = a_z.partial_cmp(&b_z).unwrap().then(a_vec.partial_cmp(&b_vec).unwrap().reverse());
        return zcmp;
    };

    // results.sort_by(&sortfn);
    // results = results.iter().take(TAKE).cloned().collect();
    // for (i, result) in results.iter().enumerate() {
    //     println!("{}: {:?}", i, result);
    // }

    // THIS ONLY DOES 13 DIGITS :facepalm:
    while results[0].1.len() < 14 {
        // Add another digit to the first thousand (or whatever is in TAKE) lowest,
        // then find the lowest again.
        let mut next_digit_results = vec![];

        for (_, res_vec) in &results {
            let x = *aluvm.vars.get("x").unwrap();
            let rg = match x {
                1..=9 => {
                    println!("x is {}", x);
                    x..=x
                },
                _ => 1..=9
            };
            
            for i in rg {
                // for i2 in 1..=9 {
                    // for i3 in 1..=9 {
                    //     for i4 in 1..=9 {
                            // for i5 in 1..=9 {
                                let mut input = res_vec.clone();
                                // input.append(&mut vec![i, i2]); //, i3, i4]); //, i5]);
                                input.push(i);

                                aluvm.reset();
                                match aluvm.run_on_input(&input) {
                                    Ok(_) => (),
                                    Err(_) => ()
                                };
                            
                                let z = *aluvm.vars.get("z").unwrap();

                                next_digit_results.push((z, input));                                        
                            // }
                    //     }
                    // }
                // }
            }
        }

        next_digit_results.sort_by(&sortfn);
        results = next_digit_results.iter().take(TAKE).cloned().collect();

        println!("\n---\n");
        for (i, result) in results.iter().take(10).enumerate() {
            println!("{}: {:?}", i, result);
        }
    }
}

fn run_part1(mut aluvm: Aluvm) {
    // Part 1 -- run MONAD to find the largest 14-digit number
    // let mut curr = 100000000000000i64; // highest 14-digit number

    // let mut curr = 99999987172995i64; // Already check everything higher than this.

    // let mut curr = 11111111111111i64;
    

    // let mut curr = 99999893100000i64; // I *think* I correctly checked values higher than this, but not sure.
    // let mut curr = 99999891951648i64;
    // let mut curr = 99999877539328i64;

    let mut curr = 811_1_1_37_1111111i64;
    let mut lowest_z = i64::MAX;

    loop {
        // curr -= 13;
        curr += 1;

        // I just want to search things starting with 811_1_1_37_
        if curr > 811_1_1_37_9999999i64 {
            println!("Not found.");
            break; // None found.
        }

        let input = get_as_vec(curr);
        if input.iter().any(|&n| n == 0) // ||
        //    input[2] != input[3] ||
        //    input[5] != input[4] + 2 ||
        //    input[6] != input[5] - 4 ||
        //    input[6] != input[3] + 2 ||
        //    input[6] != input[1] + 6
        {
            // if (curr % 25 == 0) {
            //     println!("{}", curr);
            // }
            continue;
        }

        aluvm.reset();
        aluvm.run_on_input(&get_as_vec(curr));

        let z = *aluvm.vars.get("z").unwrap();
        if z == 0 {
            break;
        }

        if z < lowest_z {
            println!("Lowest so far is {} for {}", aluvm.get_z_base26(), curr);
            lowest_z = z;
        }

        // if (curr % 2451) == 0 {
        //     break;
        // }
    }

    println!("Smallest MONAD-accepted model number: {}", curr);
}

// fn gen_vec(a: i64) -> Vec<i64> {
//     *111377
//     0123456

//     input[2] != input[3] ||
//     input[5] != input[4] + 2 ||
//     input[6] != input[5] - 4 ||
//     input[6] != input[3] + 2 ||
//     input[6] != input[1] + 6
// }

fn get_as_vec(a: i64) -> Vec<i64> {
    let mut result = vec![];
    let mut rem = a;
    let mut i = 14;
    while rem > 0 && i > 0 {
        result.push(rem % 10);
        rem /= 10;
        i -= 1;
    }

    result.reverse();
    return result;
}

// A virtual machine for the ALU
#[derive(Debug)]
struct Aluvm {
    vars: Variables,
    inst: Vec<Inst>
}

impl Aluvm {
    fn new() -> Aluvm {
        Aluvm {
            vars: Self::default_vars(),
            inst: vec![]
        }
    }

    fn load_instructions(&mut self) {
        for line in io::stdin().lock().lines() {
            let linestr = line.unwrap();
            let parts: Vec<&str> = linestr.split(" ").collect();

            let a: Vname = parts[1].to_string();

            let instruction = match parts[0] {
                "inp" => Inst::Inp(a),
                "add" => Inst::Add(a, Val::from_str(parts[2])),
                "mul" => Inst::Mul(a, Val::from_str(parts[2])),
                "div" => Inst::Div(a, Val::from_str(parts[2])),
                "mod" => Inst::Mod(a, Val::from_str(parts[2])),
                "eql" => Inst::Eql(a, Val::from_str(parts[2])),
                _ => panic!("Unkonwn instruction")
            };
            self.inst.push(instruction);
        }
    }

    fn run_on_input(&mut self, vals: &Vec<i64>) -> Result<(), ()> {
        let mut valiter = vals.iter();

        for i in &self.inst {
            if Self::run_inst(&mut self.vars, i.clone(), &mut valiter).is_err() {
                return Err(());
            }
        }

        return Ok(());
    }

    fn brk_on_eq(&mut self, instructions: &mut std::slice::Iter<Inst>, valiter: &mut std::slice::Iter<i64>) -> Result<(), ()> {
        for i in instructions {
            if let Inst::Eql(_, _) = i {
                break;
            }

            if Self::run_inst(&mut self.vars, i.clone(), valiter).is_err() {
                return Err(());
            }
        }
        return Ok(());
    }

    fn run_inst(vars: &mut Variables, i: Inst, valiter: &mut std::slice::Iter<i64>) -> Result<(), ()> {
        match i {
            Inst::Inp(vname) => {
                match valiter.next() {
                    Some(val) => {
                        // println!("State on input {}:", val);
                        // Self::print(&vars);
                        vars.insert(vname, *val)
                    },
                    None => return Err(())
                };
            },
            Inst::Add(vname, val) => {
                let a = *vars.get(&vname).unwrap();
                let b = Self::get_val(val, vars);
                vars.insert(vname, a + b);
            },
            Inst::Mul(vname, val) => {
                let a = *vars.get(&vname).unwrap();
                let b = Self::get_val(val, vars);
                vars.insert(vname, a * b);
            },
            Inst::Div(vname, val) => {
                let a = *vars.get(&vname).unwrap();
                let b = Self::get_val(val, vars);
                vars.insert(vname, a / b);
            },
            Inst::Mod(vname, val) => {
                let a = *vars.get(&vname).unwrap();
                let b = Self::get_val(val, vars);
                vars.insert(vname, a % b);
            },
            Inst::Eql(vname, val) => {
                let a = *vars.get(&vname).unwrap();
                let b = Self::get_val(val, vars);

                // Debugging if b is not 0
                if b != 0 {
                    println!("x is {}, other is {}", vars.get("x").unwrap(), b);
                }

                vars.insert(vname, if a == b { 1 } else { 0 });
            }
        }

        return Ok(());
    }

    fn get_val(val: Val, vars: &Variables) -> i64 {
        match val {
            Val::Lit(v) => v,
            Val::Var(vname) => *vars.get(&vname).unwrap()
        }
    }

    fn reset(&mut self) {
        self.vars = Self::default_vars();
    }

    fn default_vars() -> Variables {
        return [
            ("x".to_string(), 0),
            ("y".to_string(), 0),
            ("w".to_string(), 0),
            ("z".to_string(), 0)
        ].iter().cloned().collect();
    }

    fn printvars(&self) {
        Self::print(&self.vars);
    }

    fn print(vars: &Variables) {
        println!("w: {}, x: {}, y:{}, z:{}",
            vars.get("w").unwrap(),
            vars.get("x").unwrap(),
            vars.get("y").unwrap(),
            vars.get("z").unwrap(),
        );
    }

    fn get_z_base26(&self) -> String {
        let mut z = *self.vars.get("z").unwrap();
        let mut res = vec![];

        while z > 0 {
            let c = ((z % 26) as u8 + 'A' as u8) as char;
            res.push(c);
            z /= 26;
        }

        res.reverse();
        return res.iter().collect::<String>();
    }
}

type Vname = String;
type Variables = HashMap<Vname, i64>;

#[derive(Clone, Debug)]
enum Val {
    Lit(i64),
    Var(Vname)
}

impl Val {
    fn from_str(s: &str) -> Val {
        if let Ok(number) = s.parse::<i64>() {
            return Val::Lit(number);
        }

        return Val::Var(s.to_string());
    }
}

#[derive(Clone, Debug)]
enum Inst {
    Inp(Vname),
    Add(Vname, Val),
    Mul(Vname, Val),
    Div(Vname, Val),
    Mod(Vname, Val),
    Eql(Vname, Val)
}


