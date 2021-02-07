use std::any::type_name;

fn println_type_of<T>(_: &T) {
    println!("{}", type_name::<T>());
}

fn main() {
    let tup = (500, 6.4, 1);
    println!("The value of the tuple is: {:?}", tup);
    print!("The inferred type of the tuple is: ");
    println_type_of(&tup);
    println!("This is how to pretty-print it:");
    println!("{:#?}", tup);

    let tup: (i64, f64, u8) = (500, 6.4, 1);
    println!("The new shadowed value of the tuple is: {:?}", tup);
    print!("The forced/declared type of the tuple is: ");
    println_type_of(&tup);
    println!("");

    // Destructuring via pattern matching
    let (a, b, c) = tup;
    println!("The value of 'a' is: {}", a);
    println!("The value of 'b' is: {}", b);
    println!("The value of 'c' is: {}", c);

    // Or you can use the index accessor methods on the tuple obj to do the same
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("The value of 'five_hundred' is: {}", five_hundred);
    println!("The value of 'six_point_four' is: {}", six_point_four);
    println!("The value of 'one' is: {}", one);

    // Test some shadowing and mutation charcteristics of Tuples
    // Start with shadowing
    let tup_ptr = &tup as *const (i64, f64, u8);
    let tup_addr = tup_ptr as usize;
    println!("The current location of the tuple is: {}", tup_addr);
    let tup: (i64, f64, u8) = (200, 4.6, 3);
    let tup_ptr = &tup as *const (i64, f64, u8);
    let tup_addr = tup_ptr as usize;
    println!("The tuple has been shadowed to now be value: {:?}", tup);
    println!("The current location of the tuple is: {}", tup_addr);
    println!("*As one would expect, shadowing changes the tuple address.*");
    // Now, for mutations
    let mut tup: (i64, f64, u8) = (500, 6.4, 1);
    let tup_ptr = &tup as *const (i64, f64, u8);
    let tup_addr = tup_ptr as usize;
    println!("The tuple is shadowed back to {:?} but is now mutable.", tup);
    println!("The current address of the tuple is {}", tup_addr);
    tup = (200, 4.6, 3);
    let tup_ptr = &tup as *const (i64, f64, u8);
    let tup_addr = tup_ptr as usize;
    println!("TThe tuple is mutated to {:?}, now.", tup);
    println!("The current address of the tuple is {}", tup_addr);
    println!("*The address did not change.*");
    println!("*This indicates that Rust must not reallocate for Tuples because the types are known at compile time.*");
    // What if there is a String?
    // // There's some rules for tuples, apparently. Only the last tuple item is
    // // permitted to be a dynamically-sized type (such as a str), and I'm not
    // // sure how to get the ptr for a tuple which contains a dynamically sized
    // // type. TODO: research online how to get the pointer to such a tuple.
    // let mut tup: (i64, f64, &str) = (500, 6.4, "Hello!");
    // let tup_ptr = tup as *const (i64, f64, &str);
    // let tup_addr = tup_ptr as usize;
    // println!("The tuple is shadowed back to {:?} and is mutable but also contains a str", tup);
    // println!("The current address of the tuple is {}", tup_addr);
    // tup = (200, 4.6, "Goodbye!");
    // let tup_ptr = tup as *const (i64, f64, &str);
    // let tup_addr = tup_ptr as usize;
    // println!("TThe tuple is mutated to {:?}, now.", tup);
    // println!("The current address of the tuple is {}", tup_addr);
    // println!("*The address did not change.*");
    // println!("*This indicates that Rust must not reallocate for Tuples because the types are known at compile time.*");
}
