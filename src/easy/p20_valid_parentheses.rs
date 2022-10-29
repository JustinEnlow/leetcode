// Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
//     An input string is valid if:
//         Open brackets must be closed by the same type of brackets.
//         Open brackets must be closed in the correct order.
// 
//     Constraints:
//         1 <= s.length <= 104
//         s consists of parentheses only '()[]{}'.





// leetcode specified function signature
pub fn is_valid(s: String) -> bool {
    for (i, char) in s.chars().enumerate(){
        if i == 0{
            if char == '}' || char == ']' || char == ')'{
                return false;
            }
        }

        let next_char = match s.chars().nth(i.saturating_add(1)){
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



// more generic function signature. same exact code
// pub fn is_valid(s: &str) -> bool{}





fn _do_test(example: &str, s: &str, expected: bool){
    let result = is_valid(s.to_string());
    assert!(
        result == expected,
        "\n{example:?}: input = {s:?}, Expected = {expected:?} but got {result:?}\n"
    );
}

#[test]
fn tests(){
    // Example 1:
    //    Input: s = "()"
    //    Output: true
    _do_test("ex_1", "()", true);

    // Example 2:
    //    Input: s = "()[]{}"
    //    Output: true
    _do_test("ex_2", "()[]{})", true);

    // Example 3:
    //    Input: s = "(]"
    //    Output: false
    _do_test("ex_3", "(]", false);
}