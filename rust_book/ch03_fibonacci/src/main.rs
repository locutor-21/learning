use std::io;

fn calculate_fibonacci(n: u32) -> u64 {
    if n < 3{
        1
    } else {
        let mut i = 1;
        let mut j = 1;

        for _ in 1..n-1 {
            let aux = j;
            j += i;
            i = aux;
        }
        j
    }
}

fn main() {
    loop {
        println!("Input n:");

        let mut n = String::new();

        io::stdin().read_line(&mut n)
            .expect("Failed to read line");

        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match n {
            1 => println!("The {}st Fibonacci number is: {}", n, calculate_fibonacci(n)),
            2 => println!("The {}nd Fibonacci number is: {}", n, calculate_fibonacci(n)),
            _ => println!("The {}th Fibonacci number is: {}", n, calculate_fibonacci(n)),
        }
    }
}
