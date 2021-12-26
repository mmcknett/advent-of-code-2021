use std::collections::HashMap;
use std::io::{self, BufRead};

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


    run_part1(aluvm);
}

fn run_part1(mut aluvm: Aluvm) {
    // Part 1 -- run MONAD to find the largest 14-digit number
    // let mut curr = 100000000000000i64; // highest 14-digit number

    // let mut curr = 99999987172995i64; // Already check everything higher than this.

    // let mut curr = 11111111111111i64;

    // let mut curr = 99999893100000i64; // I *think* I correctly checked values higher than this, but not sure.
    // let mut curr = 99999891951648i64;
    let mut curr = 99999877539328i64;
    loop {
        // curr -= 13;
        curr -= 1;
        if curr < 11111111111111i64 {
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
        aluvm.run_on_input(get_as_vec(curr));

        let z = *aluvm.vars.get("z").unwrap();
        if z == 0 {
            break;
        }

        if (curr % 17576 == 0) {
            println!("{} results in {:#x}", curr, z);
        }

        // if (curr % 2451) == 0 {
        //     break;
        // }
    }

    println!("Largest MONAD-accepted model number: {}", curr);
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

fn get_as_vec(a: i64) -> [i64; 14] {
    let mut result = [0; 14];
    let mut rem = a;
    let mut i = 14;
    while rem > 0 && i > 0 {
        result[i - 1] = rem % 10;
        rem /= 10;
        i -= 1;
    }

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

    fn run_on_input(&mut self, vals: [i64; 14]) {
        let mut valiter = vals.iter();

        for i in &mut self.inst {
            Self::run_inst(&mut self.vars, i.clone(), &mut valiter);
        }
    }

    fn run_inst(vars: &mut Variables, i: Inst, valiter: &mut std::slice::Iter<i64>) {
        match i {
            Inst::Inp(vname) => {
                vars.insert(vname, *valiter.next().unwrap());
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
                vars.insert(vname, if a == b { 1 } else { 0 });
            }
        }
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
        println!("w: {}, x: {}, y:{}, z:{}",
            self.vars.get("w").unwrap(),
            self.vars.get("x").unwrap(),
            self.vars.get("y").unwrap(),
            self.vars.get("z").unwrap(),
        );
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
