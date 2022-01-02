use std::io;

fn main() {
    loop {
        let mut mode_input = String::new();
        println!("Which mode would you like to use:");
        println!("1. Integer to Character");
        println!("2. Character to integer");
        println!("0. Exit");
        println!("Input the number to enter that mode.");

        io::stdin()
            .read_line(&mut mode_input)
            .expect("Failed to read line");

        let mode: u8 = mode_input.trim().parse().expect("Please type a number");

        if mode == 1 {
            int_to_char();
        } else if mode == 2 {
            char_to_int();
        } else if mode == 0 {
            break;
        }
    }
}

fn int_to_char() {
    println!("Enter integer to convert to a character:");
}

fn char_to_int() {
    println!("Enter character to convert to an integer:");
    let mut char_input = String::new();
    io::stdin()
        .read_line(&mut char_input)
        .expect("Failed to read line");
    let c: char = char_input.trim().parse().expect("Please type a character");
    println!("{} is represented as {} as a decimal integer.", c, c as u32);
}
