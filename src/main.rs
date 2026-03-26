use rfd::FileDialog;

//Note to self: consider switching to using threads to handle key inputs (kind of buggy right now)
fn main() {

    loop {

        let file = FileDialog::new().pick_file(); 

        if let Some(file) = file { 
            println!("file path: {:?}", file); 
            break; 
        } else { 
            println!("Something went wrong when processing the file. Please try again or click q to quit.");
            let mut input = String::new(); 
            std::io::stdin().read_line(&mut input).expect("Failed to read input");
            let input = input.trim().to_lowercase(); 

            if input == "q" { 
                println!("Exiting the program...");
                std::process::exit(0);
            }
        }
    }


}
