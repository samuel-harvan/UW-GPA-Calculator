use rfd::FileDialog;
use std::fs; 
use std::io::{Error, ErrorKind}; // testing
mod check_key; 

fn main() {

    check_key::run_thread(); 

    loop {

        let file = FileDialog::new().pick_file(); 

        if let Some(file) = file { 
            println!("file path: {:?}", file); 
            
            let mut content = fs::read_to_string(file); 
            //content = Err(Error::new(ErrorKind::InvalidInput, "fake error"));

            match content { 
                Ok(_content) => {
                    break; 
                }
                Err(_) => {
                    println!("Something went wrong when processing the file. Please try again or click q to quit.");
                }
            }

        } else { 
            println!("Something went wrong when processing the file. Please try again or click q to quit.");
        }

    }

}
