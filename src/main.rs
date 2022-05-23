//Let's convert a number grade to a letter grade, using if and elif statements and comparisons. First have the user enter a number representing the grade (0-100). Then convert the number grade to a letter grade.

use std::io;

mod ltr_grade;

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

    println!("Letter Grade: {}", ltr_grade::ltr_grade(num_grade));
}
