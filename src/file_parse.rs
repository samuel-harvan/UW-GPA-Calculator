use rfd::FileDialog;
use std::io::{BufRead, BufReader, SeekFrom, Seek};
use std::io; 
use pdf_extract::extract_text; 
use std::io::Write; 
use tempfile::tempfile; 
//use std::io::{Error, ErrorKind}; for testing purposes

#[derive(Debug)]
pub struct GradeEntry { 
    pub credits: f32, 
    pub grade: f32,
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

    //println!("Selected file: {:?}", path);

    // Conversion from binary to string and then to a reader 
    // Note: the expected file is a .pdf file (other formats will not work properly) 
    let text = extract_text(&path).expect("Failed to extract text from the PDF file.");
    let mut temp_file = tempfile().expect("Failed to create temp file"); 
    temp_file.write_all(text.as_bytes()).expect("Failed to write to temp file");
    temp_file.seek(SeekFrom::Start(0))?; 
    let reader = BufReader::new(temp_file); 


    for line in reader.lines() { 

        let line = line?; 


        if (line.contains("In GPA") && line.contains("Earned")) || (line.trim().is_empty()) { 
            
            //println!("End of course table"); 
            course_table = false; 
            continue; 


        } else if line.contains("Course") && line.contains("Earned") && line.contains("Grade") {
            
            //println!("Start of course table"); 
            course_table = true; 
            continue; 

        } else if course_table {

            //println!("Goin thru course table");

            let elements = line.split_whitespace().collect::<Vec<&str>>();

            //println!("Elements: {:#?}", elements);

            if elements.len() < 6 {

                continue; 

            }

            let grade_info = GradeEntry {
                credits: elements[elements.len() - 2].parse::<f32>().unwrap_or(0.0),
                grade: elements[elements.len() - 1].parse::<f32>().unwrap_or(0.0), 
            }; 

            grades.push(grade_info); 
        
        } else { 

            continue; 

        }

    }

    Ok(grades)

}