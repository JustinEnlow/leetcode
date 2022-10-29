// Given a string s consisting of words and spaces, return the length of the last word in the string.
// A word is a maximal substring consisting of non-space characters only. 

//Constraints:
//    1 <= s.length <= 104
//    s consists of only English letters and spaces ' '.
//    There will be at least one word in s.





pub fn length_of_last_word(s: String) -> i32 {
    for (i, word) in s.split_whitespace().rev().enumerate(){
        if i == 0{
            return word.len() as i32
        }
    }

    0    
}

//pub fn length_of_last_word(input: &str) -> usize{
//    for (i, word) in input.split_whitespace().rev().enumerate(){
//        if i == 0{
//            return word.len()
//        }
//    }
//
//    0
//}





fn _do_test(example: &str, s: &str, expected: i32){
    let result = length_of_last_word(s.to_string());
    assert!(
        result == expected,
        "\n{example:?}: input = {s:?}, Expected = {expected:?} but got {result:?}\n"
    )
}

#[test]
fn tests(){
    // Example 1:
    //     Input: s = "Hello World"
    //     Output: 5
    //     Explanation: The last word is "World" with length 5.
    _do_test("ex_1", "Hello World", 5);

    // Example 2:
    //     Input: s = "   fly me   to   the moon  "
    //     Output: 4
    //     Explanation: The last word is "moon" with length 4.
    _do_test("ex_2", "   fly me   to   the moon  ", 4);

    // Example 3:
    //     Input: s = "luffy is still joyboy"
    //     Output: 6
    //     Explanation: The last word is "joyboy" with length 6.
    _do_test("ex_3", "luffy is still joyboy", 6);
}