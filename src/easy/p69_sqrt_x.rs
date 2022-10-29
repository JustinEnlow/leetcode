// Given a non-negative integer x, return the square root of x rounded down to 
// the nearest integer. The returned integer should be non-negative as well.

// You must not use any built-in exponent function or operator.
// For example, do not use pow(x, 0.5) in c++ or x ** 0.5 in python.





pub fn my_sqrt(x: i32) -> i32 {
    //let idk = x as f64;
    //idk.sqrt() as i32
    // can't use built in function

    for i in 2..i32::max_value(){   //0 and 1 can be left out because they dont square
        if i * i == x{
            return i;
        }
        else if i * i > x{
            return i - 1;
        }
    }

    0
}





fn _do_test(example: &str, x: i32, expected: i32){
    let result = my_sqrt(x);
    assert!(
        result == expected,
        "\n{example:?}: input = {x:?} Expected = {expected:?} but got {result:?}\n"
    );
}

#[test]
fn tests(){
    //Example 1:
    //    Input: x = 4
    //    Output: 2
    //    Explanation: The square root of 4 is 2, so we return 2.
    _do_test("ex_1", 4, 2);

    //Example 2:
    //    Input: x = 8
    //    Output: 2
    //    Explanation: The square root of 8 is 2.82842..., and since we round it down to the nearest integer, 2 is returned.
    _do_test("ex_2", 8, 2);
}