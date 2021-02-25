use std::io; // Use for read input
use std::io::Write; // Use for print and input in line
    
fn main() {
    println!("Programm return value of n-Fibonacchi number");
    loop {
        print!("Please input number: ");
        io::stdout().flush().unwrap(); 
        let mut n = String::new();
        io::stdin()
            .read_line(&mut n)
            .expect("Faild to read line");
        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Recursive method: {}", rec_fib(n));
        println!("Dynamic programming method: {}", dinamic_fib(n));
    }


}

fn rec_fib(n: u32) -> u64 {
    if n > 2 {
        rec_fib(n-2) + rec_fib(n-1)
    } else {
        1
    }
}

fn dinamic_fib(n: u32) -> u64 {
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _ in 0..n {
        let t = b;
        b = a+b;
        a = t;
    }
    a
}

