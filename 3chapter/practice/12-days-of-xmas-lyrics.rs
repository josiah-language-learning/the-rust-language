fn convert_num_to_word(num: usize) -> &'static str {
    let number_words = [
        "a",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve"
    ];

    number_words[num]
}

fn convert_num_to_nth_word(num: usize) -> &'static str {
    let nth_words = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "nineth",
        "tenth",
        "eleventh",
        "twelveth"
    ];

    nth_words[num]
}

fn print_christmas_gifts(iteration: usize) {
    let xmas_gifts = [
        "partridge in a pear tree",
        "turtle doves",
        "French hens",
        "calling birds",
        "golden rings",
        "geese a-laying",
        "swans a-swimming",
        "maids a-milking",
        "ladies dancing",
        "lords a-leaping",
        "pipers piping",
        "drummers drumming"
    ];

    if iteration == 0 {
        println!("\t{} {}!", convert_num_to_word(0), xmas_gifts[0]);
    } else {
        for i in (1..iteration).rev() {
            println!("\t{} {}", convert_num_to_word(i), xmas_gifts[i]);
        }

        println!(
            "\tand {} {}!",
            convert_num_to_word(0),
            xmas_gifts[0]
        )
    }
}

fn main() {
    for i in 0..12 {
        println!(
            "On the {} day of Christmas, my true love gave to me...",
            convert_num_to_nth_word(i)
        );
        print_christmas_gifts(i + 1);
        println!("");
    }
}
