// Given a string s consisting of words and spaces, return the length of the last word in the string.
// A word is a maximal substring consisting of non-space characters only. 

//Constraints:
//    1 <= s.length <= 104
//    s consists of only English letters and spaces ' '.
//    There will be at least one word in s.

pub fn length_of_last_word(input: &str) -> usize{
    for (i, word) in input.split_whitespace().rev().enumerate(){
        if i == 0{
            return word.len()
        }
    }

    0
}





// Example 1:
//     Input: s = "Hello World"
//     Output: 5
//     Explanation: The last word is "World" with length 5.
#[test]
fn ex_1() {
    let s = "Hello World";
    assert!(5 == length_of_last_word(s));
}

// Example 2:
//     Input: s = "   fly me   to   the moon  "
//     Output: 4
//     Explanation: The last word is "moon" with length 4.
#[test]
fn ex_2(){
    let s = "   fly me   to   the moon  ";
    assert!(4 == length_of_last_word(s));
}

// Example 3:
//     Input: s = "luffy is still joyboy"
//     Output: 6
//     Explanation: The last word is "joyboy" with length 6.
#[test]
fn ex_3(){
    let s = "luffy is still joyboy";
    assert!(6 == length_of_last_word(s));
}