//// use
//
//// unfinished
//pub fn longest_substring_without_repeating_characters(input: &str) -> usize{
//    let mut substring_start: usize = 0;
//    let mut substring_end: usize = 0;
//    let mut last_char: Option<char> = None;
//
//    for (i, char) in input.chars().enumerate(){
//        match last_char{
//            Some(prev_char) => {
//                if char == prev_char{
//                    substring_start = i;
//                    last_char = Some(char);
//                }
//                else{
//                    substring_end = i;
//                    last_char = Some(char);
//                }
//            },
//            None => last_char = Some(char)
//        }
//
//        println!("start: {}, end: {}, last_char: {}", substring_start, substring_end, last_char.unwrap());
//    }
//
//    substring_start.abs_diff(substring_end)
//}
//
//
//
//
//    
//// Given a string s, find the length of the longest substring without repeating characters.
//
////Constraints:
////    0 <= s.length <= 5 * 104
////    s consists of English letters, digits, symbols and spaces.
//
////Example 1:
////    Input: s = "abcabcbb"
////    Output: 3
////    Explanation: The answer is "abc", with the length of 3.
//#[test]
//fn ex_1(){
//    let input = "abcabcbb";
//    assert!(longest_substring_without_repeating_characters(input) == 3);
//}
//
////Example 2:
////    Input: s = "bbbbb"
////    Output: 1
////    Explanation: The answer is "b", with the length of 1.
//#[test]
//fn ex_2(){
//    let input = "bbbbb";
//    assert!(longest_substring_without_repeating_characters(input) == 1);
//}
//
////Example 3:
////    Input: s = "pwwkew"
////    Output: 3
////    Explanation: The answer is "wke", with the length of 3.
////    Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
//#[test]
//fn ex_3(){
//    let input = "pwwkew";
//    assert!(longest_substring_without_repeating_characters(input) == 3);
//}