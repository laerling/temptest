use std::collections::HashMap;
use std::{env,fmt};
use std::fmt::{Display,Formatter};
use std::ops::{Add, Sub, AddAssign, SubAssign};

#[derive(Copy,Clone,PartialEq)]
enum StackEntry {
    Num(u64),
    Term((u64,u64)),
}

impl Display for StackEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            StackEntry::Num(n) => write!(f, "{}", n),
            StackEntry::Term((m,n)) => write!(f, "({} {})", m, n),
        }
    }
}

impl Add for StackEntry {
    type Output = Self;
    fn add(self, other: Self) -> StackEntry {
        match (self,other) {
            (StackEntry::Num(a),
             StackEntry::Num(b)) => StackEntry::Num(a+b),
            _ => panic!("Can only add Num(_) with Num(_)"),
        }
    }
}

impl Sub for StackEntry {
    type Output = Self;
    fn sub(self, other: Self) -> StackEntry {
        match (self,other) {
            (StackEntry::Num(a),
             StackEntry::Num(b)) => StackEntry::Num(a-b),
            _ => panic!("Can only add Num(_) with Num(_)"),
        }
    }
}

impl AddAssign for StackEntry {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for StackEntry {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

fn print_stack(stack: &Vec<StackEntry>) {
    let stack_string: Vec<String> = stack.iter()
        .filter(|n_or_t| match **n_or_t {
            StackEntry::Num(_) => true, _ => false
        }).map(|n| n.to_string()).collect();
    println!("{}", stack_string.join(" "));
}

fn main() {

    // TODO remember solutions to sub-problems and look them up

    // parse args
    let mut args = env::args().skip(1);
    let arg1 = args.next().expect("Too few arguments");
    let m: u64 = arg1.parse().unwrap_or_else(
        |_| panic!("First argument cannot be parsed as number: {}", arg1));
    let arg2 = args.next().expect("Too few arguments");
    let n: u64 = arg2.parse().unwrap_or_else(
        |_| panic!("Second argument cannot be parsed as number: {}", arg2));
    println!("m={} n={}", m, n);

    // prepare
    let mut stack = vec![StackEntry::Num(m), StackEntry::Num(n)];
    let solutions: HashMap<(u64,u64),u64> = HashMap::new();

    loop {

        // print current iteration
        print_stack(&stack);

        // if only one element is left we're done
        let len = stack.len();
        if len == 1 {
            break;
        }

        // else calculate Ackermann function
        let m = stack[len-2];
        let n = stack[len-1];
        if m == StackEntry::Num(0) {
            // A(0, n) = n+1
            stack[len-1] += StackEntry::Num(1);
            stack.remove(len-2);
        } else if n == StackEntry::Num(0) {
            // A(m, 0) = A(m-1, 1)
            stack[len-2] -= StackEntry::Num(1);
            stack[len-1] = StackEntry::Num(1);
        } else {
            // A(m, n) = A(m-1, A(m, n-1))
            stack[len-1] -= StackEntry::Num(1);
            stack.insert(len-2, m-StackEntry::Num(1));
        }
    }
}
