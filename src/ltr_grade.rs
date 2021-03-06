/*Find the specific letter grade (A+, B-, etc).
 You can check for more specific ranges using if statements,
 or use modulus % to get the ones-digit to set another string
 to '+', '-', or ' '. Then you can concatenate that string
 with your grade string.
*/

pub fn ltr_grade(user_input: u8) -> String {
    let grade_equiv: String;
    if user_input > 100 {
        grade_equiv = "Extra credit not allowed".to_string();
        grade_equiv
    } else if user_input > 89 && !user_input > 100 {
        grade_equiv = "A".to_string();
        grade_equiv
    } else if user_input > 79 && !user_input > 90 {
        grade_equiv = "B".to_string();
        grade_equiv
    } else if user_input > 69 && !user_input > 80 {
        grade_equiv = "C".to_string();
        grade_equiv
    } else if user_input > 59 && !user_input > 70 {
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

pub struct SpecificGrade {
    pub plus: String,
    pub minus: String,
}

#[test]
fn test_grade_a() {
    for grade in 90..=100 {
        assert_eq!(ltr_grade(grade), "A");
    }
}
#[test]
fn test_grade_b() {
    for grade in 80..=89 {
        assert_eq!(ltr_grade(grade), "B");
    }
}
#[test]
fn test_grade_c() {
    for grade in 70..=79 {
        assert_eq!(ltr_grade(grade), "C");
    }
}
#[test]
fn test_grade_d() {
    for grade in 60..=69 {
        assert_eq!(ltr_grade(grade), "D");
    }
}
#[test]
fn test_grade_f() {
    for grade in 1..=59 {
        assert_eq!(ltr_grade(grade), "F");
    }
}

#[test]
fn test_all_overages() {
    for i in 101..=255 {
        assert_eq!(ltr_grade(i), "Extra credit not allowed");
    }
}

#[test]
fn test_refined_grades() {
    let refined_grade: SpecificGrade = SpecificGrade {
        plus: String::from("+"),
        minus: String::from("-"),
    };
    for i in 0..=100 {
        match i % 10 {
            7 | 8 | 9 => assert_eq!(refined_grade.plus, "+".to_string()),
            4 | 5 | 6 => println!("{}", ltr_grade(i)),
            0 | 1 | 2 | 3 => assert_eq!(refined_grade.minus, "-".to_string()),
            _ => assert_eq!("Special Case", "Special Case"),
        };
    }
}
