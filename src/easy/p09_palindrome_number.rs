// Given an integer x, return true if x is palindrome integer.
//    An integer is a palindrome when it reads the same backward as forward.
//    For example, 121 is a palindrome while 123 is not.

//    Constraints:
//        -231 <= x <= 231 - 1





pub fn is_palindrome(x: i32) -> bool {
    let stringified_input = x.to_string();
    let reversed_input: String = stringified_input.chars().rev().collect();

    if stringified_input == reversed_input{
        return true;
    }

    false    
}





fn _do_test(example: &str, x: i32, expected: bool){
    let result = is_palindrome(x);
    assert!(
        result == expected,
        "\n{example:?}: input = {x:?}, Expected = {expected:?} but got {result:?}\n"
    )
}

#[test]
fn tests(){
    //Example 1:
    //    Input: x = 121
    //    Output: true
    //    Explanation: 121 reads as 121 from left to right and from right to left.
    _do_test("ex_1", 121, true);
    
    //Example 2:
    //    Input: x = -121
    //    Output: false
    //    Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.    
    _do_test("ex_2", -121, false);
    
    //Example 3:
    //    Input: x = 10
    //    Output: false
    //    Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
    _do_test("ex_3", 10, false);
}