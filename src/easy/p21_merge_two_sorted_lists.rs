// You are given the heads of two sorted linked lists list1 and list2.
//     Merge the two lists in a one sorted list. The list should be made by splicing together the nodes of the first two lists.
//     Return the head of the merged linked list.
// 
//     Constraints:
//         The number of nodes in both lists is in the range [0, 50].
//         -100 <= Node.val <= 100
//         Both list1 and list2 are sorted in non-decreasing order.

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
                println!("val1: {}, val2: {}", val1, val2);
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
    for val in result_list.iter(){
        println!("result: {}", val);
    }
    //

    result_list
}





    // Example 1:
    //    Input: list1 = [1,2,4], list2 = [1,3,4]
    //    Output: [1,1,2,3,4,4]
    #[test]
    fn ex_1(){
        let mut list1 = LinkedList::new();
        list1.push_back(1);
        list1.push_back(2);
        list1.push_back(4);

        let mut list2 = LinkedList::new();
        list2.push_back(1);
        list2.push_back(3);
        list2.push_back(4);

        let mut result = merge_two_sorted_lists(list1, list2);

        assert!(result.pop_front().unwrap() == 1);
        assert!(result.pop_front().unwrap() == 1);
        assert!(result.pop_front().unwrap() == 2);
        assert!(result.pop_front().unwrap() == 3);
        assert!(result.pop_front().unwrap() == 4);
        assert!(result.pop_front().unwrap() == 4);
        // 
        assert!(result.pop_front().is_none());
    }

    // Example 2:
    //    Input: list1 = [], list2 = []
    //    Output: []
    #[test]
    fn ex_2(){
        let list1 = LinkedList::new();

        let list2 = LinkedList::new();

        assert!(merge_two_sorted_lists(list1, list2).pop_front().is_none());
    }

    // Example 3:
    //    Input: list1 = [], list2 = [0]
    //    Output: [0]
    #[test]
    fn ex_3(){
        let list1 = LinkedList::new();

        let mut list2 = LinkedList::new();
        list2.push_back(0);

        let mut result = merge_two_sorted_lists(list1, list2);

        assert!(result.pop_front().unwrap() == 0);
        //
        assert!(result.pop_front().is_none());
    }