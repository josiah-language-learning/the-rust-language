fn plus_one(x: i32) -> i32 {  // -> i32 tells rustc this function returns a value
    x + 1;  // but this ';' tells rustc this is a statement and not a return val
            // This causes rustc to throw an error because there's nothing after
            // the x + 1; statement which provides a return value that matches
            // the declared i32 type in the function signature.
}

fn main() {
    let six = plus_one(5);
    println!("5 + 1 is: {}", six);
}
