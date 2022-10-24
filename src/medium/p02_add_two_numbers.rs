//use std::collections::LinkedList;
//
//
//
//// unfinished
//pub fn add_two_numbers(list1: LinkedList<u32>, list2: LinkedList<u32>) -> LinkedList<u32>{
//    let mut list1_string = String::new();
//    for i in list1{
//        let item_str = i.to_string();
//        list1_string.push_str(&item_str);
//    }
//
//    println!("{}", list1_string);
//
//    let mut list2_string = String::new();
//
//    let result = LinkedList::new();
//
//    result
//}
//
//
//
//
//
//    /*
//    You are given two non-empty linked lists representing two non-negative integers. 
//    The digits are stored in reverse order, and each of their nodes contains a single digit. 
//    Add the two numbers and return the sum as a linked list.
//
//    You may assume the two numbers do not contain any leading zero, except the number 0 itself.
//    */
//    #[test]
//    fn ex_1() {
//        let mut l1 = LinkedList::new();
//        l1.push_front(3);
//        l1.push_front(4);
//        l1.push_front(2);
//
//        let mut l2 = LinkedList::new();
//        l2.push_front(4);
//        l2.push_front(6);
//        l2.push_front(5);
//
//        // 342 + 465 = 807
//        let mut result = add_two_numbers(l1, l2);
//        assert!(result.pop_front().unwrap() == 8);
//        assert!(result.pop_front().unwrap() == 0);
//        assert!(result.pop_front().unwrap() == 7);
//        // not defined as requirement in leetcode, but i want to verify this is the case
//        assert!(result.pop_front().is_none());
//    }
//
//    #[test]
//    fn ex_2(){
//        let mut l1 = LinkedList::new();
//        l1.push_front(0);
//
//        let mut l2 = LinkedList::new();
//        l2.push_front(0);
//
//        let mut result = add_two_numbers(l1, l2);
//        assert!(result.pop_front().unwrap() == 0);
//        // not defined as requirement in leetcode, but i want to verify this is the case
//        assert!(result.pop_front().is_none());
//    }
//
//    #[test]
//    fn ex_3(){
//        let mut l1 = LinkedList::new();
//        l1.push_front(9);
//        l1.push_front(9);
//        l1.push_front(9);
//        l1.push_front(9);
//        l1.push_front(9);
//        l1.push_front(9);
//        l1.push_front(9);
//
//        let mut l2 = LinkedList::new();
//        l2.push_front(9);
//        l2.push_front(9);
//        l2.push_front(9);
//        l2.push_front(9);
//
//        // 9999999 + 9999 = 10009998
//        let mut result = add_two_numbers(l1, l2);
//        assert!(result.pop_front().unwrap() == 1);// 1
//        assert!(result.pop_front().unwrap() == 0);// 0
//        assert!(result.pop_front().unwrap() == 0);// 0
//        assert!(result.pop_front().unwrap() == 0);// 0
//        assert!(result.pop_front().unwrap() == 9);// 9
//        assert!(result.pop_front().unwrap() == 9);// 9
//        assert!(result.pop_front().unwrap() == 9);// 9
//        assert!(result.pop_front().unwrap() == 8);// 8
//        // not defined as requirement in leetcode, but i want to verify this is the case
//        assert!(result.pop_front().is_none());// none
//    }