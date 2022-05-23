pub fn ltr_grade(user_input: u8) -> String {
    let grade_equiv: String;
    if user_input > 100 {
        grade_equiv = "Extra credit not allowed".to_string();
        grade_equiv
    } else if user_input > 89 && user_input > 100 {
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

#[test]
fn test_grade_a() {
    assert_eq!(ltr_grade(91), "A");
}
#[test]
fn test_grade_b() {
    assert_eq!(ltr_grade(85), "B");
}
#[test]
fn test_grade_c() {
    assert_eq!(ltr_grade(75), "C");
}
#[test]
fn test_grade_d() {
    assert_eq!(ltr_grade(65), "D");
}
#[test]
fn test_grade_f() {
    assert_eq!(ltr_grade(55), "F");
}
#[test]
fn test_grade_not_a() {
    assert_eq!(ltr_grade(101), "input not recognized");
}
#[test]
fn test_grade_not_b() {
    assert_eq!(ltr_grade(111), "input not recognized");
}
#[test]
fn test_grade_not_c() {
    assert_eq!(ltr_grade(121), "input not recognized");
}
#[test]
fn test_grade_not_d() {
    assert_eq!(ltr_grade(131), "input not recognized");
}
#[test]
fn test_grade_not_f() {
    assert_eq!(ltr_grade(141), "input not recognized");
}
