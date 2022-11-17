// You are given the heads of two sorted linked lists list1 and list2.
// Merge the two lists in a one sorted list. The list should be made by 
// splicing together the nodes of the first two lists.
// Return the head of the merged linked list.
// 
// Constraints:
//     The number of nodes in both lists is in the range [0, 50].
//     -100 <= Node.val <= 100
//     Both list1 and list2 are sorted in non-decreasing order.





// leetcode defined function signature

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
//pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//
//}





use std::collections::LinkedList;

pub fn merge_two_sorted_lists(list1: LinkedList<i32>, mut list2: LinkedList<i32>) -> LinkedList<i32>{
    let mut result_list = LinkedList::new();

    match (list1.len() > 0, list2.len() > 0){
        (true, true) => {
            for &val1 in list1.iter(){
                let val2 = match list2.pop_front(){
                    Some(val) => val,
                    None => break,
                };
            
                // debug
                //println!("val1: {}, val2: {}", val1, val2);
                //
            
                if val1 > val2{
                    result_list.push_back(val2);
                    result_list.push_back(val1);
                }else{
                    result_list.push_back(val1);
                    result_list.push_back(val2);
                }
            }
        },
        (true, false) => {
            for val in list1{
                result_list.push_back(val)
            }
        },
        (false, true) => {
            for val in list2{
                result_list.push_back(val)
            }
        },
        (false, false) => {},
    }

    // debug
    //for val in result_list.iter(){
    //    println!("result: {}", val);
    //}
    //

    result_list
}





fn _do_test(example: &str, list1: LinkedList<i32>, list2: LinkedList<i32>, expected: LinkedList<i32>){
    let result = merge_two_sorted_lists(list1.clone(), list2.clone());
    assert!(
        result == expected,
        "\n{example:?}: Input = {list1:?} and {list2:?}, expected {expected:?} but got {result:?}\n"
    )
}

#[test]
fn tests(){
    // Example 1: Input: list1 = [1,2,4], list2 = [1,3,4], Output: [1,1,2,3,4,4]
    _do_test("ex_1", LinkedList::from([1, 2, 4]), LinkedList::from([1, 3, 4]), LinkedList::from([1, 1, 2, 3, 4, 4]));

    // Example 2: Input: list1 = [], list2 = [], Output: []
    _do_test("ex_2", LinkedList::from([]), LinkedList::from([]), LinkedList::from([]));

    // Example 3: Input: list1 = [], list2 = [0], Output: [0]
    _do_test("ex_3", LinkedList::from([]), LinkedList::from([0]), LinkedList::from([0]));
}