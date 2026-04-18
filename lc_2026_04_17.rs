// fn main() {
//     // let mut example: i32 = 1000000;
//     // let n_num: i32 = reverser(&mut example);
//     // println!("In Main: {}", n_num);
//     ab_dif();
    
// }

// // fn reverser(num:&mut i32) -> i32{
// //     if *num % 10 == 0{
// //         while *num % 10 == 0{
// //             *num /= 10;
// //         }
// //     }
    
// //     let mut num_str: String = num.to_string();
// //     num_str = num_str.chars().rev().collect();
    
// //     let r_num = num_str.parse().unwrap();
// //     println!("In Reverser: {}", num_str);
// //     r_num
// // }

// fn ab_dif(){
//     let num1: i32 = 6;
//     let num2: i32 = 5;
//     println!("Absolute Difference between {} and {} is {} and {}",num1, num2, num1.abs_diff(num2), num2.abs_diff(num1) );
// }
fn min_mirror_pair_distance(nums: Vec<i32>) -> i32 {

    let mut dist = nums.len();

    for i in 0..nums.len(){

        let mut cur_num  =  nums[i];
        let com_num = reverser(&mut cur_num);

        for j in (i+1)..nums.len(){

            if com_num == nums[j]{

                let diff = i.abs_diff(j);
                if diff < dist{
                    dist = diff;
                }
            }
        }
    }

    if dist == nums.len(){
        return -1;
    } 
    return dist as i32;
    
}

fn reverser(num:&mut i32) -> i32{
    // println!("Reverser Working: {}", *num);

    if *num % 10 == 0{
        while *num % 10 == 0{
            *num /= 10;
        }
    }
    
    let mut num_str: String = num.to_string();
    num_str = num_str.chars().rev().collect();
    
    let r_num = num_str.parse().unwrap();
    // println!("Reverser Giving: {}", r_num);
    r_num
}

fn main(){
    let nums: Vec<i32> = vec![12,21,45,33,54];
    println!("{}",min_mirror_pair_distance(nums));
}

// use std::collections::HashMap;

// impl Solution {
//     pub fn min_mirror_pair_distance(nums: Vec<i32>) -> i32 {
//         let mut seen: HashMap<i32, usize> = HashMap::new();
//         let mut min_dist = usize::MAX;

//         for (j, &num) in nums.iter().enumerate() {
//             // Check if current number matches a previously reversed number
//             if let Some(&i) = seen.get(&num) {
//                 min_dist = min_dist.min(j - i);
//             }

//             // Store reverse(nums[j]) for future matches
//             let rev = Self::reverse(num);
//             seen.insert(rev, j);
//         }

//         if min_dist == usize::MAX {
//             -1
//         } else {
//             min_dist as i32
//         }
//     }

//     fn reverse(mut num: i32) -> i32 {
//         let mut rev = 0;
//         while num > 0 {
//             rev = rev * 10 + num % 10;
//             num /= 10;
//         }
//         rev
//     }
// }