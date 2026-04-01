use crate::file_parse::GradeEntry; 

pub fn find_gpa(mut grades: Vec<GradeEntry>) -> f32 {

    let mut total_gpa = 0.0; 

    let scale = vec! [
        (90.0, 4.00),
        (85.0, 3.90),
        (80.0, 3.70),
        (77.0, 3.30),
        (73.0, 3.00),
        (70.0, 2.70),
        (67.0, 2.30),
        (63.0, 2.00),
        (60.0, 1.70),
        (57.0, 1.30),
        (53.0, 1.00),
        (50.0, 0.70), 
        (47.0, 0.0)
    ];

    let mut total_credits = 0.0; 

    for class_mark in grades.iter_mut() {

        total_credits += class_mark.credits;

        for (percent, gpa) in &scale {
            if class_mark.grade >= *percent {
                class_mark.grade = *gpa; 
                break;
            }
        }
    }

    if total_credits == 0.0 { 
            return 0.0; 
    }

    for gpa_score in grades.iter() { 

        total_gpa += gpa_score.grade * gpa_score.credits; 
    }

    total_gpa /= total_credits; 

    return total_gpa; 

}
            
        
    
    
