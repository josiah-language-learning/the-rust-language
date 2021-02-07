fn main () {
    let a = [0, 1, 2, 3, 4];
    let index = 5;
    // The below index out of bounds error is caught by the compiler, even if
    // the index to access is provided by a variable.  This is pretty neat.  The
    // book actually flubbed this part and claimed this would result in a
    // runtime error, but it doesn't.  The only time it would, I suspect, is
    // when the value of the variable cannot be determined at compile time (such
    // as in the case of a mutable variable storing an index provided from the
    // cli arguments).
    println!("The 6th element of array 'a' is: {}", a[index]);
}
