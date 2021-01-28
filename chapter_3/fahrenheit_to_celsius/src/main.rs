use std::io; // Use for read input
use std::io::Write; // Use for print and input in line

fn main() {
    println!("Fahrenheit and Celsius conversation!");
    println!("If you want convert 10F into Celsius input 10F");
    println!("And if you want convert 10C into Fahrenheit input 10C");
    loop {
        print!("Input your temperature: ");
        io::stdout().flush().unwrap(); // Use for print and input in line
        let mut t = String::new(); // variable for temperature input
        io::stdin() // Read input -> t
            .read_line(&mut t)
            .expect("Failed to read line");
        let t = t.trim(); // Del /n
        let (t, scale) = t.split_at(t.len() - 1); // Split input to temperature and scale
                                                  // Conver temperature to float
        let t: f32 = match t.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Wrong value of temperature!");
                continue;
            }
        };
        // Convert temperature from F to C or vice versa
        match scale {
            "F" => {
                let t = (t - 32.) * (5. / 9.);
                println!("{}C", t);
            }
            "C" => {
                let t = t * (9. / 5.) + 32.;
                println!("{}F", t)
            }
            _ => println!("Wrong temperature scale!"),
        };
    }
}
