// Write a function to find the longest common prefix string amongst an array of strings.
//     If there is no common prefix, return an empty string "".
// 
//     Constraints:
//         1 <= strs.length <= 200
//         0 <= strs[i].length <= 200
//         strs[i] consists of only lowercase English letters.





// leetcode specified function signature
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut shortest_word = String::new();
    for word in &strs{
        if shortest_word == ""{
            shortest_word = word.to_string();
        }
        else{
            if word.len() < shortest_word.len(){
                shortest_word = word.to_string()
            }
        }
    }

    let mut should_quit = false;
    let mut last_index_reached = 0;

    for i in 0..shortest_word.len(){
        if should_quit == true{ break }

        for word in &strs{
            if word.chars().nth(i).unwrap() == shortest_word.chars().nth(i).unwrap(){
                last_index_reached = i.saturating_sub(1);
            }else{
                if i == 0{
                    return "".to_string();
                }
                else{
                    should_quit = true
                }
            }
        }
    }

    let shortest_word_as_string = String::from(shortest_word);
    let longest_prefix_string = &shortest_word_as_string[0..=last_index_reached];

    longest_prefix_string.to_string()
}





fn _do_test(example: &str, strs: &[String], expected: &str){
    let result = longest_common_prefix(strs.to_vec());
    assert!(
        result == expected,
        "\n{example:?}: input = {strs:?}, Expected = {expected:?} but got {result:?}\n"
    );
}

#[test]
fn tests(){
    // Example 1:
    //    Input: strs = ["flower","flow","flight"]
    //    Output: "fl"
    _do_test(
        "ex_1",
        &["flower".to_string(), "flow".to_string(), "flight".to_string()], 
        "fl"
    );

    // Example 2:
    //    Input: strs = ["dog","racecar","car"]
    //    Output: ""
    //    Explanation: There is no common prefix among the input strings.
    _do_test(
        "ex_2",
        &["dog".to_string(), "racecar".to_string(), "car".to_string()], 
        ""
    );
}