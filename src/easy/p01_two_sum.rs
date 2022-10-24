use std::collections::HashMap;

pub fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)>{
    //solution_one(nums, target)
    //solution_two(nums, target)
    //solution_three(nums, target)
    solution_four(nums, target)
}



// brute force. checking all values more than once
pub fn solution_one(nums: &[i32], target: i32) -> Option<(usize, usize)>{
    for (i, &outer_val) in nums.iter().enumerate(){
        for (j, &inner_val) in nums.iter().enumerate(){
            if i == j{
                continue
            }
            if outer_val + inner_val == target{
                return Some((i, j));
            }
        }
    }
    None
}

pub fn solution_two(nums: &[i32], target: i32) -> Option<(usize, usize)>{
    for i in 0..nums.len(){
        for j in i+1..nums.len(){
            if nums[i] + nums[j] == target{
                return Some((i, j));
            }
        }
    }
    None
}

pub fn solution_three(nums: &[i32], target: i32) -> Option<(usize, usize)>{
    let mut hashmap = HashMap::new();
    for (i, &val) in nums.iter().enumerate(){
        hashmap.insert(val, i);
    }

    for (i, &val) in nums.iter().enumerate(){
        let remainder = target - val;
        match hashmap.get(&remainder){
            Some(&j) => {
                if i != j{
                    return Some((i, j))
                }
            },
            None => continue
        }
    }
    None
}

pub fn solution_four(nums: &[i32], target: i32) -> Option<(usize, usize)>{
    let mut hashmap = HashMap::new();
    
    for (i, &val) in nums.iter().enumerate(){
        let remainder = target - val;
        match hashmap.get(&remainder){
            Some(&j) => {
                return Some((j, i))
            },
            None => {}
        }
        hashmap.insert(val, i);
    }
    
    None
}





#[test]
fn ex_1(){
    let nums = [2, 7, 11, 15];
    let target = 9;
    assert!(Some((0, 1)) == two_sum(&nums, target));
}

#[test]
fn ex_2(){
    let nums = [3, 2, 4];
    let target = 6;
    assert!(Some((1, 2)) == two_sum(&nums, target));
}

#[test]
fn ex_3(){
    let nums = [3, 3];
    let target = 6;
    assert!(Some((0, 1)) == two_sum(&nums, target));
}

#[test]
fn my_example(){
    let nums = [1, 2, 3];
    let target = 7;
    assert!(None == two_sum(&nums, target));
}