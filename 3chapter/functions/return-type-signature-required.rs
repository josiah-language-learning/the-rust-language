// The missing type signature for the return value causes a compiler error.
fn plus_two(x: i32) {
    x + 2
}

fn main() {
    // The following println statement failes at the call to plus_two()
    println!("5 + 2 is: {}", plus_two());
}
