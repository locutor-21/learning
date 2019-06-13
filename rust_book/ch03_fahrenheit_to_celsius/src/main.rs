use std::io;

fn main() {
    loop {
        let mut f = String::new(); 
    
        println!("Input in Fahrenheit:");

        io::stdin().read_line(&mut f)
            .expect("Failed to read");

        let f: i32 = match f.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let c: i32 = (f-32) * 5 / 9;

        println!("Output in Celsius: {}", c);
    }
}
