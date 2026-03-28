use rfd::FileDialog;
use std::fs::File; 
use std::io::{BufRead, BufReader};
use std::io; 
//use std::io::{Error, ErrorKind}; for testing purposes
pub struct GradeEntry { 
    credits: f32, 
    grade: i32,
}

pub fn find_grades() -> io::Result<Vec<GradeEntry>> {

    let path = loop {

        let file = FileDialog::new().pick_file(); 

        if let Some(file) = file { 
            break file; 
        } else { 
            println!("Something went wrong when processing the file. Please try again or click q to quit.");
        }

    };

    let mut grades = Vec::new(); 
    let mut course_table = false; 
    let content: File = File::open(path)?; 

    for line in BufReader::new(content).lines() { 

        let line = line?; 

        if (line.contains("In GPA") && line.contains("Earned")) || (line.trim().is_empty()) { 
            
            course_table = false; 
            continue; 

        } else if line.contains("Course") && line.contains("Credits") && line.contains("Grade") {
            
            course_table = true; 
            continue; 

        } else if course_table {

            let elements = line.split_whitespace().collect::<Vec<&str>>();

            let grade_info = GradeEntry {
                credits: elements[4].parse::<f32>().unwrap_or(0.0),
                grade: elements[5].parse::<i32>().unwrap_or(0), 
            }; 

            grades.push(grade_info); 
        
        } else { 

            continue; 

        }

    }

    Ok(grades)

}