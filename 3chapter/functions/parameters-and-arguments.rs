fn main() {
    another_function("Hello", "Josiah", 2);
}

fn another_function(greeting: &str, name: &str, num_greets: i32) {
    for _ in 0..num_greets {
        println!("{}, {}!", greeting, name);
    }
}
