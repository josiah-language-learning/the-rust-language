use std::io;

fn f_to_c(deg_f: f64) -> f64 {
    (deg_f - 32.0) * 5.0 / 9.0
}

fn c_to_f(deg_c: f64) -> f64 {
    deg_c * 9.0 / 5.0 + 32.0
}

fn split_temp_str(temp_str: &str) -> (f64, char) {
    let temp_unit_char = temp_str.chars().last().unwrap();

    let temp_num: f64 = match &temp_str[..temp_str.len() - 1].parse() {
        Ok(num) => *num,
        Err(msg) => {
            println!("PARSE ERROR: {}", msg);
            panic!(
                "Please enter a valid integer component of the \
                    temperature."
            );
        }
    };

    (temp_num, temp_unit_char)
}

fn get_output_unit_char(temp_unit_char: char) -> char {
    match temp_unit_char {
        'F' => 'C',
        'C' => 'F',
        _ => panic!("Unrecognized temperature unit!")
    }
}

fn perform_conversion(temp_num: f64, temp_unit_char: char) -> String {
    let output_unit_char = get_output_unit_char(temp_unit_char);
    let output_temp_num = match temp_unit_char {
        'F' => f_to_c(temp_num),
        'C'  => c_to_f(temp_num),
        _ => panic!("Unrecognized temperature unit!")
    };

    format!("{}{}", output_temp_num, output_unit_char)
}

fn main() {
    let prompt = "Enter the degrees after the form [0-9]+[F|C] \
                  (for example, '70F', or '30C')...";
    loop {
        println!("{}", prompt);

        let mut temp_str = String::new();
        io::stdin()
            .read_line(&mut temp_str)
            .expect("Failed to read-in the input temperature.");
        temp_str = temp_str.trim().to_string();

        let (temp_num, temp_unit_char) = split_temp_str(&temp_str);

        println!(
            "The converted result is {} equals {}.\n",
            temp_str,
            perform_conversion(temp_num, temp_unit_char)
        )
    }
}
