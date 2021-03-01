// Program print substrings if exists
use std::io;

fn main() {
    loop {
        println!("Write you string: ");
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Faild to readline"); // mutable references to s
        println!("First word: {}", first_word(&s));
        // references to s as parametr for first_word
        // we want use s after this function
        println!("Second word (if exists): {}", second_word(&s)); // exactly the same
        println!("All word(substrings):");
        all_substrings(&s);
    }
}
fn first_word(s: &str) -> &str {
    let b = s.as_bytes();
    for (i, &item) in b.iter().enumerate() {
        // iter is a method that returns each element in a collection
        // enumerate wraps the result of iter and returns each element as part of a tuple
        if item == b' ' {
            return &s[..i]; // the slice is &str values
        }
    }
    &s
}
fn second_word(s: &str) -> &str {
    let b = s.as_bytes();
    let mut n = 0;
    for (i, &item) in b.iter().enumerate() {
        if item == b' ' && n == 0 {
            n = i;
        } else if item == b' ' || i == b.len() - 1 && n != 0 {
            return &s[n + 1..i];
        }
    }
    "-" // String Literals Are Slices and are &str values
}

fn all_substrings(s: &str) {
    for i in s.split(' ') {
        println!("{}", i);
    }
}
