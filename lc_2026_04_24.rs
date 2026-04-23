//Character To Integer
// impl Solution {
//     pub fn my_atoi(s: String) -> i32 {

//         let mut sign: char = '+';
//         let mut nums: Vec<char> = Vec::new();
//         let mut trim_s: String = s.trim().to_string();
//         let fst_char: char = trim_s.chars().next().unwrap();

//          if fst_char == '-'{
//             sign = '-';
//             println!("{}", sign);
//          }

//          if fst_char == '-' || fst_char == '+'{
//             trim_s = trim_s[1..].to_string();
//          }

//          for ch in trim_s.chars(){
//             if matches!(ch, '0'..='9'){
//                 nums.push(ch);
//             }
//             else{
//                 break
//             }
//          }

//          if nums.is_empty(){
//              0
//          }
//          else{
//             let result: String = nums.iter().collect();
//             let result: i32 = result.parse().unwrap();

//             if sign == '-'{
//                 -1*result
//             }
//             else{
//                 result
//             }

//          }
        
//     }
// }
//Optimized
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut i = 0;
        let n = bytes.len();

        // skip whitespace
        while i < n && bytes[i] == b' ' {
            i += 1;
        }

        // sign
        let mut sign = 1;
        if i < n && (bytes[i] == b'-' || bytes[i] == b'+') {
            if bytes[i] == b'-' {
                sign = -1;
            }
            i += 1;
        }

        // number
        let mut result: i32 = 0;
        while i < n && bytes[i].is_ascii_digit() {
            let digit = (bytes[i] - b'0') as i32;

            // overflow handling
            if result > (i32::MAX - digit) / 10 {
                return if sign == 1 { i32::MAX } else { i32::MIN };
            }

            result = result * 10 + digit;
            i += 1;
        }

        result * sign
    }
}