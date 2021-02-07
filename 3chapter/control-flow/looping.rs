fn main() {
    // using loop { ... } by itself will create an infinite loop. Since there's
    // an example of this in the Guessing Game program from Chapter 2, already,
    // I won't write another one, here.

    // Returning Values from Loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result returned from the first loop is: {}", result);

    // Conditional Loops with `while`
    let mut num = 10;

    println!("T minus {} seconds and counting", num);
    while num != 0 {
        num -= 1;
        println!("{}", num);
    }

    println!("We have LIFTOFF!!!");

    // Looping through a collection with `for`
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    println!("Looping through a collection via a while loop.");
    println!("The collection used holds 10 -> 50 stepping by 10");
    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    println!("A better way to loop through the same collection is to use for.");
    for element in a.iter() {
        println!("The value is: {}", element);
    }
}
