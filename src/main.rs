//Let's convert a number grade to a letter grade, using if and elif statements and comparisons. First have the user enter a number representing the grade (0-100). Then convert the number grade to a letter grade.

use std::io;

mod ltr_grade;

use ltr_grade::ltr_grade;
use ltr_grade::SpecificGrade;

fn main() {
    // Prompt user for number grade
    println!("Please enter your number grade: ");

    // Mutable initialized string variable; contents = ""
    let mut num_grade: String = String::new();

    io::stdin()
        .read_line(&mut num_grade)
        .expect("Failed to read line");

    let num_grade: u8 = match num_grade.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let refined_grade: SpecificGrade = SpecificGrade {
        plus: String::from("+"),
        minus: String::from("-"),
    };

    match num_grade % 10 {
        10 => println!("{}{}", ltr_grade(num_grade), refined_grade.plus),
        7 | 8 | 9 => println!("{}{}", ltr_grade(num_grade), refined_grade.plus),
        4 | 5 | 6 => println!("{}", ltr_grade(num_grade)),
        0 | 1 | 2 | 3 => println!("{}{}", ltr_grade(num_grade), refined_grade.minus),
        _ => println!("Special Case"),
    };
}
