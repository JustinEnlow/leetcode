// Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
//     An input string is valid if:
//         Open brackets must be closed by the same type of brackets.
//         Open brackets must be closed in the correct order.
// 
//     Constraints:
//         1 <= s.length <= 104
//         s consists of parentheses only '()[]{}'.

pub fn valid_parentheses(input: &str) -> bool{
    for (i, char) in input.chars().enumerate(){
        if i == 0{
            if char == '}' || char == ']' || char == ')'{
                return false;
            }
        }

        let next_char = match input.chars().nth(i.saturating_add(1)){
            Some(val) => {val},
            None => {'\0'},
        };

        match char{
            '{' => {
                if next_char != '}'{ return false; }
            },
            '[' => {
                if next_char != ']'{ return false; }
            },
            '(' => {
                if next_char != ')'{ return false; }
            },
            _ => {},
        }
    }    
    
    true
}





// Example 1:
//    Input: s = "()"
//    Output: true
#[test]
fn ex_1(){
    let s = "()";
    assert!(valid_parentheses(s) == true);
}

// Example 2:
//    Input: s = "()[]{}"
//    Output: true
#[test]
fn ex_2(){
    let s = "()[]{}";
    assert!(valid_parentheses(s) == true);
}

// Example 3:
//    Input: s = "(]"
//    Output: false
#[test]
fn ex_3(){
    let s = "(]";
    assert!(valid_parentheses(s) == false);
}