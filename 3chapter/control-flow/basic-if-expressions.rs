fn less_than_5(number: i32) {
    if number < 5 {
        println!("number <{}> is less than 5.", number);
    } else {
        println!("number <{}> is greater than or equal to 5.", number);
    }
}

fn main() {
    let number = 3;
    let another_number = 7;
    let yet_another_number = 5;

    less_than_5(number);
    less_than_5(another_number);
    less_than_5(yet_another_number);

    // Else If
    let six = 6;

    if six % 4 == 0 {  // false
        println!("'six' is divisible by 4.");
    } else if six % 3 == 0 {  // true
        println!("'six' is divisible by 3.");
    } else if six % 2 == 0 {  // will never reach this code.
        println!("'six' is divisible by 2.");
    } else {
        println!("'six' is not divisible by 4, 3, or 2.");
    }

    println!("");

    // Compare the above else if to...
    if six % 4 == 0 {  // false
        println!("'six' is divisible by 4.");
    } if six % 3 == 0 {  // true
        println!("'six' is divisible by 3.");
    } if six % 2 == 0 {  // true, and now this code will be reached as well.
        println!("'six' is divisible by 2.");
    }
}
