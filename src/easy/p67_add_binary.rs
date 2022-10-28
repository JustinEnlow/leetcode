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





//Example 1:
//    Input: a = "11", b = "1"
//    Output: "100"
#[test]
fn ex_1(){
    let a = "11".to_string();
    let b = "1".to_string();
    assert!(add_binary(a, b) == "100".to_string());
}

//Example 2:
//    Input: a = "1010", b = "1011"
//    Output: "10101"
#[test]
fn ex_2(){
    let a = "1010".to_string();
    let b = "1011".to_string();
    assert!(add_binary(a, b) == "10101".to_string());
}