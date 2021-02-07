fn main() {
    another_function("Hello", "Josiah", 2);
}

fn another_function(greeting, name, num_greets) {
    for _ in 0..num_greets {
        println!("{}, {}!", greeting, name);
    }
}
