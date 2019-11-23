use std::io;

fn main() {
    // Mode
    let mode;
    loop {
        println!("Type C or F, and Enter.");
        let mut i = String::new();
        io::stdin().read_line(&mut i)
            .expect("Failed to read line.");
        i = i.trim().to_uppercase();
        if i == "C" || i == "F" {
            mode = i;
            break;
        } else {
            continue;
        }
    }

    // Temperature
    let temp :f64;
    loop {
        println!("Please input {} temperature.", mode);
        let mut i = String::new();
        io::stdin().read_line(&mut i)
            .expect("Failed to read line.");
        temp = match i.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break;
    }
    
    // Convert
    if mode == "C" {
        let f :f64 = (temp * 9.0 / 5.0) + 32.0;
        println!("{}°C is {}°F", temp, f);
    } else if mode == "F" {
        let c :f64 = (temp - 32.0) * 5.0 / 9.0;
        println!("{}°F is {}°C", temp, c);
    } else {
        println!("An unexpected error has occurred.")
    }
}