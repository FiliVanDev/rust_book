fn main() {
    let days: [&str; 12] = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three french hens",
        "four calling birds",
        "FIVE GOLD RINGS",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    for day in 0..12 {
        let suf: &str = match day + 1 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        };
        println!(
            "\nOn the {}{} day of Christmas my true love gave to me:",
            day + 1,
            suf
        );

        for n in (0..day + 1).rev() {
            if day > 0 && n == 0 {
                print!("And ");
            }
            println!("{}", days[n])
        }
    }
}
