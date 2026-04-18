impl Solution {
    pub fn mirror_distance(n: i32) -> i32 {
        
        let mut num: i32 = n;
        n.abs_diff(Self::reverser(&mut num)) as i32

    }

    pub fn reverser(n: &mut i32)-> i32{

        if *n % 10 == 0{
            while *n % 10 == 0{
                *n /= 10;
            }
        }
        
        let mut n_str: String = n.to_string();
        n_str = n_str.chars().rev().collect();
        let ret_num = n_str.parse().unwrap();
        ret_num
    }
}

// impl Solution {
//     pub fn mirror_distance(n: i32) -> i32 {
        
//         let num: i32 = n;
//         n.abs_diff(Self::reverser(num)) as i32

//     }

//     pub fn reverser(mut num: i32) -> i32 {
//         let mut rev = 0;
//         while num > 0 {
//             rev = rev * 10 + num % 10;
//             num /= 10;
//         }
//         rev
//     }
// }