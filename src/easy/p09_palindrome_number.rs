// Given an integer x, return true if x is palindrome integer.
//    An integer is a palindrome when it reads the same backward as forward.
//    For example, 121 is a palindrome while 123 is not.
//
//    Constraints:
//        -231 <= x <= 231 - 1

pub fn palindrome_number(input: i32) -> bool{
    let stringified_input = input.to_string();
    //let mut reversed_input = String::new();
    //for c in stringified_input.chars().rev(){
    //    reversed_input.push(c)
    //}
    let reversed_input: String = stringified_input.chars().rev().collect();

    if stringified_input == reversed_input{
        return true;
    }

    false
}



//Example 1:
//    Input: x = 121
//    Output: true
//    Explanation: 121 reads as 121 from left to right and from right to left.
#[test]
fn ex_1(){
    let x = 121;
    assert!(palindrome_number(x) == true);    
}

//Example 2:
//    Input: x = -121
//    Output: false
//    Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
#[test]
fn ex_2(){
    let x = -121;
    assert!(palindrome_number(x) == false);
}

//Example 3:
//    Input: x = 10
//    Output: false
//    Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
#[test]
fn ex_3(){
    let x = 10;
    assert!(palindrome_number(x) == false);
}
