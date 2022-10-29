//You are climbing a staircase. It takes n steps to reach the top.
//Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?


//Constraints:
//    1 <= n <= 45





pub fn climb_stairs(_n: i32) -> i32 {
    0
}





fn _do_test(example: &str, n: i32, expected: i32){
    let result = climb_stairs(n);
    assert!(
        result == expected,
        "\n{example:?}: input = {n:?}, Expected = {expected:?} but got {result:?}\n"
    )
}

#[test]
fn tests(){
    //Example 1:
    //    Input: n = 2
    //    Output: 2
    //    Explanation: There are two ways to climb to the top.
    //    1. 1 step + 1 step
    //    2. 2 steps
    _do_test("ex_1", 2, 2);

    //Example 2:
    //    Input: n = 3
    //    Output: 3
    //    Explanation: There are three ways to climb to the top.
    //    1. 1 step + 1 step + 1 step
    //    2. 1 step + 2 steps
    //    3. 2 steps + 1 step
    _do_test("ex_2", 3, 3);
}