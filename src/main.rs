//Let's convert a number grade to a letter grade, using if and elif statements and comparisons. First have the user enter a number representing the grade (0-100). Then convert the number grade to a letter grade.

use std::io;

fn ltr_grade(user_input: u8) -> String {
    let grade_equiv: String;
    if user_input > 89 && !user_input > 100 {
        grade_equiv = "A".to_string();
        grade_equiv
    } else if user_input > 79 && !user_input > 90 {
        grade_equiv = "B".to_string();
        grade_equiv
    } else if user_input > 69 && !user_input > 80 {
        grade_equiv = "C".to_string();
        grade_equiv
    } else if user_input > 59 && !user_input > 70{
        grade_equiv = "D".to_string();
        grade_equiv
    } else if user_input <= 59 {
        grade_equiv = "F".to_string();
        grade_equiv
    } else {
        grade_equiv = "input not recognized".to_string();
        grade_equiv
    }
}

fn main() {
    // Prompt user for number grade
    println!("Please enter your number grade: ");

    // Mutable initialized string variable; contents = ""
    let mut num_grade: String = String::new();

    io::stdin()
        .read_line(&mut num_grade)
        .expect("Failed to read line");
    loop {
        let num_grade: u8 = match num_grade.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Letter Grade: {}", ltr_grade(num_grade));
        break;
    }
}
