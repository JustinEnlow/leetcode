// You are given a large integer represented as an integer array digits, where 
// each digits[i] is the ith digit of the integer. The digits are ordered from 
// most significant to least significant in left-to-right order. The large integer 
// does not contain any leading 0's.
// 
// Increment the large integer by one and return the resulting array of digits.
// 
// Constraints:
//     1 <= digits.length <= 100
//     0 <= digits[i] <= 9
//     digits does not contain any leading 0's.




pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();

    let least_significant = digits.len() - 1;
    for (i, &val) in digits.iter().enumerate(){
        if i == least_significant{
            match val{
                9 => {
                    match new_vec.pop(){
                        Some(val_2) => {
                            new_vec.push(val_2 + 1);  //value before least_significant + 1
                        },
                        None => {
                            new_vec.push(1);
                        }
                    }
                    new_vec.push(0);
                },
                _ => new_vec.push(val + 1)
            }
        }
        else{
            new_vec.push(val)
        }
    }
    
    println!("{:?}", new_vec);
    
    new_vec
}





fn _do_test(example: &str, digits: &[i32], expected: &[i32]){
    let result = plus_one(digits.to_vec());
    assert!(
        result == expected,
        "\n{example:?}: input = {digits:?}, Expected = {expected:?} but got {result:?}\n"
    )
}

#[test]
fn tests(){
    // Example 1:
    //     Input: digits = [1,2,3]
    //     Output: [1,2,4]
    //     Explanation: The array represents the integer 123.
    //     Incrementing by one gives 123 + 1 = 124.
    //     Thus, the result should be [1,2,4].
    _do_test("ex_1", &[1, 2, 3], &[1, 2, 4]);

    // Example 2:
    //     Input: digits = [4,3,2,1]
    //     Output: [4,3,2,2]
    //     Explanation: The array represents the integer 4321.
    //     Incrementing by one gives 4321 + 1 = 4322.
    //     Thus, the result should be [4,3,2,2].
    _do_test("ex_2", &[4, 3, 2, 1], &[4, 3, 2, 2]);

    // Example 3:
    //     Input: digits = [9]
    //     Output: [1,0]
    //     Explanation: The array represents the integer 9.
    //     Incrementing by one gives 9 + 1 = 10.
    //     Thus, the result should be [1,0].
    _do_test("ex_3", &[9], &[1, 0]);
}

// these tests don't cover all reasonable use cases for plus one
// for example, this code will not work for plus_one(Vec::from([9, 9, 9]))
// the code only deals with adding to the least significant number