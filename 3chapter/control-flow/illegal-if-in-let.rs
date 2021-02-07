fn main() {
    let condition = true;

    // rustc will throw an error on the next line because 5 and "six" are
    // different types
    let number = if condition { 5 } else { "six" };

    println!("The value of 'number' is: {}", number);
}
