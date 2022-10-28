// Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
//         Symbol       Value
//         I             1
//         V             5
//         X             10
//         L             50
//         C             100
//         D             500
//         M             1000
//     
//     For example, 2 is written as II in Roman numeral, just two ones added together. 
//     12 is written as XII, which is simply X + II. 
//     The number 27 is written as XXVII, which is XX + V + II.
//     
//     Roman numerals are usually written largest to smallest from left to right. 
//     However, the numeral for four is not IIII. Instead, the number four is written as IV. 
//     Because the one is before the five we subtract it making four. 
//     The same principle applies to the number nine, which is written as IX. 
//     There are six instances where subtraction is used:
//         I can be placed before V (5) and X (10) to make 4 and 9. 
//         X can be placed before L (50) and C (100) to make 40 and 90. 
//         C can be placed before D (500) and M (1000) to make 400 and 900.
//     
//     Given a roman numeral, convert it to an integer.
// 
//     Constraints:
//         1 <= s.length <= 15
//         s contains only the characters ('I', 'V', 'X', 'L', 'C', 'D', 'M').
//         It is guaranteed that s is a valid roman numeral in the range [1, 3999].



/// desired function signature as defined by leetcode
pub fn roman_to_int(s: String) -> i32 {
    let mut nums: Vec<i32> = Vec::new();

    let mut consume_next = false;

    for (i, char) in s.chars().enumerate(){
        if consume_next{
            consume_next = false;
        }
        else{
            match char{
                'I' => {
                    match s.chars().nth(i.saturating_add(1)){
                        Some(val) => {
                            match val{
                                'V' => {
                                    nums.push(4);
                                    consume_next = true;
                                },
                                'X' => {
                                    nums.push(9);
                                    consume_next = true;
                                },
                                _ => nums.push(1)
                            }
                        },
                        None => nums.push(1)
                    }
                },
                'V' => nums.push(5),
                'X' => {
                    match s.chars().nth(i.saturating_add(1)){
                        Some(val) => {
                            match val{
                                'L' => {
                                    nums.push(40); 
                                    consume_next = true;
                                },
                                'C' => {
                                    nums.push(90); 
                                    consume_next = true;
                                },
                                _ => nums.push(10)
                            }
                        },
                        None => nums.push(10)
                    }
                },
                'L' => nums.push(50),
                'C' => {
                    match s.chars().nth(i.saturating_add(1)){
                        Some(val) => {
                            match val{
                                'D' => {
                                    nums.push(400); 
                                    consume_next = true;
                                },
                                'M' => {
                                    nums.push(900); 
                                    consume_next = true;
                                },
                                _ => nums.push(100)
                            }
                        },
                        None => nums.push(100)
                    }
                },
                'D' => nums.push(500),
                'M' => nums.push(1000),
                _ => {},
            }
        }
    }

    // debug
    println!("{:?}", nums);
    //

    let mut sum = 0;

    for num in nums{
        sum = sum + num
    }

    // debug
    println!("{}", sum);
    //

    sum
}



// proposed better function signature. more generalized
// all function code remains the same
// pub fn roman_to_integer(x: &str) -> i32{}





//Example 1:
//    Input: s = "III"
//    Output: 3
//    Explanation: III = 3.
#[test]
fn ex_1(){
    let s = String::from("III");
    //assert!(roman_to_integer(&s) == 3);
    assert!(roman_to_int(s) == 3);
}

//Example 2:
//    Input: s = "LVIII"
//    Output: 58
//    Explanation: L = 50, V= 5, III = 3.
#[test]
fn ex_2(){
    let s = String::from("LVIII");
    //assert!(roman_to_integer(&s) == 58);
    assert!(roman_to_int(s) == 58);
}

//Example 3:
//    Input: s = "MCMXCIV"
//    Output: 1994
//    Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
#[test]
fn ex_3(){
    let s = String::from("MCMXCIV");
    //assert!(roman_to_integer(&s) == 1994);
    assert!(roman_to_int(s) == 1994);
}