// Given two binary strings a and b, return their sum as a binary string.

// Constraints:
//     1 <= a.length, b.length <= 104
//     a and b consist only of '0' or '1' characters.
//     Each string does not contain leading zeros except for the zero itself.





pub fn add_binary(a: String, b: String) -> String {
    let idk = match u8::from_str_radix(&a, 2){
        Ok(val) => val,
        Err(_) => panic!("nope")
    };
    let idk_2 = match u8::from_str_radix(&b, 2){
        Ok(val) => val,
        Err(_) => panic!("nope, again")
    };

    //println!("a: {:08b}, b: {:08b}", idk, idk_2);
    
    let sum = idk + idk_2;

    //println!("{:08b}", sum);
    //println!("{}", sum);

    String::from(format!("{:b}", sum))
}





fn _do_test(example: &str, a: &str, b: &str, expected: &str){
    let result = add_binary(a.to_string(), b.to_string());
    let expected = expected.to_string();
    assert!(
        result == expected,
        "\n{example:?}: input = {a:?} and {b:?}, Expected = {expected:?} but got {result:?}\n"
    );
}

#[test]
fn tests(){
    //Example 1:
    //    Input: a = "11", b = "1"
    //    Output: "100"
    _do_test("ex_1", "11", "1", "100");

    //Example 2:
    //    Input: a = "1010", b = "1011"
    //    Output: "10101"
    _do_test("ex_2", "1010", "1011", "10101");
}