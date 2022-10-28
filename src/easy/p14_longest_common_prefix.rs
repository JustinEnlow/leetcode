// Write a function to find the longest common prefix string amongst an array of strings.
//     If there is no common prefix, return an empty string "".
// 
//     Constraints:
//         1 <= strs.length <= 200
//         0 <= strs[i].length <= 200
//         strs[i] consists of only lowercase English letters.





// leetcode specified function signature
//pub fn longest_common_prefix(strs: Vec<String>) -> String {
//        
//}



// more generic function signature
pub fn longest_common_prefix(input: &[&str]) -> /*Option<String>*/String{     // is Option<String> preferrable here?
    // determine shortest word
    
    // for each letter in all words, if the letter at that index is the same, add to longest_prefix_string
    // if first letter of any word is different, we can return early

    // if end of shortest word is reached, and all letters match, return shortest word as longest_prefix_string
    
    let mut shortest_word = "";
    for &word in input{
        if shortest_word == ""{
            shortest_word = word;
        }
        else{
            if word.len() < shortest_word.len(){
                shortest_word = word
            }
        }
    }

    // debug
    println!("{}", shortest_word);
    //

    let mut should_quit = false;
    let mut last_index_reached = 0;

    for i in 0..shortest_word.len(){
        if should_quit == true{ break }

        for &word in input{
            if word.chars().nth(i).unwrap() == shortest_word.chars().nth(i).unwrap(){
                last_index_reached = i.saturating_sub(1);
            }else{
                if i == 0{
                    return "".to_string();
                    //return None;
                }
                else{
                    should_quit = true
                }
            }
        }
    }

    let shortest_word_as_string = String::from(shortest_word);
    let longest_prefix_string = &shortest_word_as_string[0..=last_index_reached];

    // debug
    println!("{}", longest_prefix_string);
    //

    /*Some(*/longest_prefix_string.to_string()//)
}





// Example 1:
//    Input: strs = ["flower","flow","flight"]
//    Output: "fl"
#[test]
fn ex_1(){
    let strs = ["flower", "flow", "flight"];
    assert!(longest_common_prefix(&strs)/*.unwrap()*/ == "fl".to_string());
}

// Example 2:
//    Input: strs = ["dog","racecar","car"]
//    Output: ""
//    Explanation: There is no common prefix among the input strings.
#[test]
fn ex_2(){
    let strs = ["dog", "racecar", "car"];
    assert!(longest_common_prefix(&strs) == /*None*/"".to_string());
}