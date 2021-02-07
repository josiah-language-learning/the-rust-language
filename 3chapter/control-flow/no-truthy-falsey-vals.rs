fn main() {
    let n = 7;

    if n {  // rustc throws compile error because this is not a boolean expr.
        println!("'n' is truthy!");
    } else {
        println!("'n' is falsey");
    }
}
