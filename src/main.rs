mod file_parse; 
mod input; 
mod gpa_calc; 

fn main() { 

    input::check_key(); 
    let grades = file_parse::find_grades(); 
    let gpa = gpa_calc::find_gpa(&grades); 

    println!("Success"); 
}