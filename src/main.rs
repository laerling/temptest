use std::env;

fn print_stack(stack: &Vec<u64>) {
    let stack_string: Vec<String> = stack.iter().map(|n| n.to_string()).collect();
    println!("{}", stack_string.join(" "));
}

fn main() {

    // parse args
    let mut args = env::args().skip(1);
    let arg1 = args.next().expect("Too few arguments");
    let m: u64 = arg1.parse().unwrap_or_else(
        |_| panic!("First argument cannot be parsed as number: {}", arg1));
    let arg2 = args.next().expect("Too few arguments");
    let n: u64 = arg2.parse().unwrap_or_else(
        |_| panic!("Second argument cannot be parsed as number: {}", arg2));
    println!("m={} n={}", m, n);

    // do stuff
    let mut stack = vec![m, n];
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
        if m == 0 {
            // A(0, n) = n+1
            stack[len-1] += 1;
            stack.remove(len-2);
        } else if n == 0 {
            // A(m, 0) = A(m-1, 1)
            stack[len-2] -= 1;
            stack[len-1] = 1;
        } else {
            // A(m, n) = A(m-1, A(m, n-1))
            stack[len-1] -= 1;
            stack.insert(len-2, m-1);
        }
    }
}
