use std::thread; 

pub fn check_key() { 
    thread::spawn(|| {
        loop {
            let mut input = String::new(); 
            std::io::stdin().read_line(&mut input).expect("Failed to read input"); 
            input = input.trim().to_lowercase(); 
            if input == "q" { 
                println!("Exiting the program...");
                std::process::exit(0);
            }
        }
    }); 
}

