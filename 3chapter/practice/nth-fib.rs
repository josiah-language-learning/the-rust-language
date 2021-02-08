use std::cmp::Ordering;
use std::io;

// It's recognized that this function can only return fib. numbers within the
// range of what's able to be represented by a u64. Therefore, this can only
// calculate up to and including the 93rd Fibionacci number.
fn nth_fib(n: i32) -> u64 {
    fn fib_memo(n: i32) -> u64 {
        let mut memo: [u64; 2] = [1, 1];

        for _ in 2..n {
            let last_fib = memo[1];
            memo = [last_fib, memo[0] + last_fib];
        }

        memo[1]
    }

    match n.cmp(&1) {
        Ordering::Less => panic!("Please enter an integer that's > 0."),
        Ordering::Equal => 1,
        Ordering::Greater => fib_memo(n)
    }
}

fn ask_for_n() -> i32 {
    let mut n_str = String::new();

    println!("Which Fibionacci number do you want?");

    io::stdin()
        .read_line(&mut n_str)
        .expect("Failed to read-in the value for 'n'");

    match n_str.trim().parse() {
        Ok(num) => num,
        Err(msg) => {
            println!("PARSE ERROR: {}", msg);
            panic!("Please enter an integer!");
        }
    }
}

fn main() {
    loop {
        let n = ask_for_n();
        let fib_num = nth_fib(n);

        println!("The {} Fibionacci number is: {}", n, fib_num);
    }
}
