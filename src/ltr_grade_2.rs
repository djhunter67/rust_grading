/*Find the specific letter grade (A+, B-, etc).
 You can check for more specific ranges using if statements,
 or use modulus % to get the ones-digit to set another string
 to '+', '-', or ' '. Then you can concatenate that string
 with your grade string.
*/

struct Specific_Grade {
    a_plus: &str = "A+",
    a_norm: &str = "A",
    a_minus: &str = "A-",
    
    b_plus: &str = "B+",
    b_norm: &str = "B",
    b_minus: &str = "B-",
    
    c_plus: &str = "C+",
    c_norm: &str = "C",
    c_minus: &str = "C-",
    
    d_plus: &str = "D+",
    d_norm: &str = "D",
    d_minus: &str = "D-",
    
    f_plus: &str = "F+",
    f_norm: &str = "F",
    f_minus: &str = "F-",
}
