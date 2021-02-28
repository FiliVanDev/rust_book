// Program print the first substring if it exists
use std::io;

fn main() {
    loop {
        println!("Write you string: ");
        let mut s = String::new();
        io::stdin()
            .read_line(&mut s)
            .expect("Faild to readline");
        println!("First word: {}", first_word(&s));
        println!("Second word (if exists): {}", second_word(&s));
    }
}
fn first_word(s: &str) -> &str {
    let b = s.as_bytes();
    for (i, &item) in b.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
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
        } else if item == b' ' || i == b.len()-1 && n != 0 {
            return &s[n+1..i]
        }  
    }
    "-"
}
