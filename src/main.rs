mod file_parse; 
mod input; 
mod gpa_calc; 

fn main() { 

    input::check_key(); 
    let grades = file_parse::find_grades(); 
    let grades = grades.expect("Failed to parse grades from the file."); 
    let gpa = gpa_calc::find_gpa(grades); 

    println!("Success"); 
}