/*
 * Shadowing is not as dangerous as mutating because the shadowed variable
 * is limited to its scope.  For example, changing a variable using 'mut'
 * in a fuction changes that variable's value everywhere in the program,
 * including outside that function's scope and all the way up the call
 * stack wherever that variable exists (in the worst case scenario, the
 * variable is declared/defined in the global scope).
 *
 * Shadowing, however, only changes that variable's value within the
 * current scope.  So the caller doesn't suffer a potentially surprising
 * mutation of the variable's value, but the function can redefine the
 * variable for itself and for its callees.  This is generally easier to
 * reason about for the programmer since the change is limited to the
 * current scope that the function body has direct control over.
 *
 * A second important consideration is I suspect mut holds onto the same
 * space in memory and just replaces the old value with the new in that
 * spot in the RAM (unless it can't like the new value takes up more space
 * than the old).  In general, this should translate to slightly better
 * performance over shadowing since memory doesn't need to be reallocated
 * and pointers don't need to be destroyed and recreated.  I'd expect
 * shadowing, however, to completely destroy, deallocate, recreate, and
 * reallocate resources, because it's treating the assignment as a brand
 * new variable.  The only difference here from a regular non-shadowing
 * assignment is that shadowing destroys the ability to reference the
 * old value (so likely Rust releases it) since the old and new variables
 * share the same alias.
 */
fn main() {
    let x = 5;
    println!("x = 5");
    println!("ROOT SCOPE: x is: {}\n", x);

    let x = x + 1;
    println!("x + 1");
    println!("ROOT SCOPE: x has been redefined/shadowed to be: {}\n", x);

    {
        // create a block with its own sub-scope
        let x = x * 3;
        println!("==>\tx * 3");
        println!(
            "\tCHILD SCOPE: x has been redefined/shadowed to be: {}\n",
            x
        );
    }

    let x = x * 2;
    println!("x * 2");
    println!(
        "ROOT SCOPE: x has been redefined/shadowed yet again to be: {}\n",
        x
    );

    let x = "Hello, world!";
    println!(
        "ROOT SCOPE: x has been redefined/shadowed to a different type: {}\n",
        x
    );

    // Testing how addressing works with shadowing.
    let x_ptr = x.as_ptr();
    let x_addr = x_ptr as usize;
    println!("x is currently at address: {}", x_addr);
    let x = "Hello, world."; // changed the ! to a .
    let x_ptr = x.as_ptr();
    let x_addr = x_ptr as usize;
    println!("x is now shadowed to: {}", x);
    println!("x is currently at address: {}", x_addr);
    println!("*Shadowing changed the address*\n");

    let mut x = "Hello, world!";
    let x_ptr = x.as_ptr();
    let x_addr = x_ptr as usize;
    println!("x is shadowed back to: '{}', but is now mut.", x);
    println!("x is currently at address: {}", x_addr);
    x = "Goodbye, Bob!";
    let x_ptr = x.as_ptr();
    let x_addr = x_ptr as usize;
    println!("x is mutated to: {}", x);
    println!("x is currently at address: {}", x_addr);
    println!("*apparently mutating strings still changes the pointer addr*\n");

    let mut x = 5;
    let x_ptr = &x as *const i32;
    let x_addr = x_ptr as usize;
    println!("x is shadowed to: '{}', is still mut.", x);
    println!("x is currently at address: {}", x_addr);
    x = 10;
    let x_ptr = &x as *const i32;
    let x_addr = x_ptr as usize;
    println!("x is mutated to: {}", x);
    println!("x is currently at address: {}", x_addr);
    println!("*mutating did NOT change the address in the case of an int*\n");

    let x = 5;
    let x_ptr = &x as *const i32;
    let x_addr = x_ptr as usize;
    println!("x is shadowed back to: '{}', but is now mut.", x);
    println!("x is currently at address: {}", x_addr);
    let x = 10;
    let x_ptr = &x as *const i32;
    let x_addr = x_ptr as usize;
    println!("x is shadowed to: {}", x);
    println!("x is currently at address: {}", x_addr);
    println!("*shadowing DID change the address in the case of an int*\n");

    println!("CONCLUSION: Strings and Lists must be handled in a special way.");
}
