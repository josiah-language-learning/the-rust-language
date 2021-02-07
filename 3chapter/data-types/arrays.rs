fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("The array 'a' is: {:?}", a);
    println!("This is how to pretty-print it:");
    println!("{:#?}", a);

    // months of the year is a good array candidate because it's unlikely
    // to need to change size or be appended to or have elements removed.
    // Types are written providing both the internal data type and the array
    // length (the number of items).  rustc is able to infer the type signature
    // of the following array, but it's provided explicitly so that it can be
    // seen how array type signatures should be written.
    let months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December"
    ];
    println!("The months of the year are:\n{:#?}", months);

    // Accessing array elements is pretty similar to Python
    let first_month = months[0];
    let last_month = months[11];
    println!("The first month of the year is: {}", first_month);
    println!("The last month of the year is: {}", last_month);
}
